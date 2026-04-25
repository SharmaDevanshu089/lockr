# Lockr — Technical Documentation

> **Stack:** Tauri v2 · SvelteKit v2 · Rust · TypeScript · AES-256-GCM  
> **Platform target:** Windows (Mica transparency effect; other platforms fall back gracefully)

---

## Table of Contents

1. [Architecture Overview](#1-architecture-overview)
2. [Project Structure](#2-project-structure)
3. [Backend — `src-tauri/`](#3-backend--src-tauri)
   - 3.1 [Entry Points — `main.rs` & `lib.rs`](#31-entry-points--mainrs--librs)
   - 3.2 [Data Structures](#32-data-structures)
   - 3.3 [Tauri Commands (IPC API)](#33-tauri-commands-ipc-api)
   - 3.4 [Cryptographic Engine](#34-cryptographic-engine)
   - 3.5 [Dependencies (`Cargo.toml`)](#35-dependencies-cargotoml)
   - 3.6 [Build & Configuration (`tauri.conf.json`)](#36-build--configuration-tauriconfjson)
4. [Frontend — `src/`](#4-frontend--src)
   - 4.1 [Static Config (`+layout.ts`)](#41-static-config-layoutts)
   - 4.2 [Global Styles (`app.css`)](#42-global-styles-appcss)
   - 4.3 [Utility Module (`lib/utils.ts`)](#43-utility-module-libutilsts)
   - 4.4 [Shared Components (`lib/components/`)](#44-shared-components-libcomponents)
5. [Routes & Pages](#5-routes--pages)
   - 5.1 [`/` — Home (`+page.svelte`)](#51---home-pagesvelte)
   - 5.2 [`/encrypt` — Encrypt Hub](#52-encrypt--encrypt-hub)
   - 5.3 [`/encrypt/file` — Encrypt File Workflow](#53-encryptfile--encrypt-file-workflow)
   - 5.4 [`/decrypt` — Decrypt Hub](#54-decrypt--decrypt-hub)
   - 5.5 [`/decrypt/file` — Decrypt File Workflow](#55-decryptfile--decrypt-file-workflow)
6. [UI State Machines](#6-ui-state-machines)
   - 6.1 [Encrypt File — State Machine](#61-encrypt-file--state-machine)
   - 6.2 [Decrypt File — State Machine](#62-decrypt-file--state-machine)
7. [IPC Data Flow — End-to-End](#7-ipc-data-flow--end-to-end)
   - 7.1 [Encryption Flow](#71-encryption-flow)
   - 7.2 [Decryption Flow](#72-decryption-flow)
8. [Cryptographic Design](#8-cryptographic-design)
9. [Window & Visual System](#9-window--visual-system)
10. [Debug Logging System](#10-debug-logging-system)
11. [Known Limitations & TODOs](#11-known-limitations--todos)

---

## 1. Architecture Overview

Lockr is a **native desktop application** that wraps a Svelte SPA inside a Tauri shell. The architecture cleanly separates concerns into two layers:

```
┌──────────────────────────────────────────────────────────┐
│                   FRONTEND (WebView)                      │
│   SvelteKit SPA  ·  flowbite-svelte  ·  bits-ui          │
│   app.css (WinUI-inspired design system)                  │
│                                                          │
│   Routes:  /  →  /encrypt  →  /encrypt/file             │
│                  /decrypt  →  /decrypt/file              │
└──────────────────────┬───────────────────────────────────┘
                       │  Tauri IPC (invoke / command)
┌──────────────────────▼───────────────────────────────────┐
│                   BACKEND (Rust)                          │
│   lib.rs  ·  AES-256-GCM  ·  rfd (file dialogs)         │
│   Tauri Commands:                                        │
│     open_file_dialog    generate_key    get_resulting_dir │
│     final_encryption    final_decryption                 │
│     read_nounce_bytes   open_in_explorer                 │
│     open_folder_dialog  get_file_path                    │
└──────────────────────────────────────────────────────────┘
```

**Key design choices:**
- **No server-side rendering.** SvelteKit runs in SPA mode (`ssr = false`, `prerender = true`) because Tauri ships the built static bundle directly; there is no Node server.
- **File I/O lives entirely in Rust.** The frontend never touches the filesystem directly; all disk operations go through Tauri commands.
- **Crypto lives entirely in Rust.** Key generation, cipher init, encryption, and decryption are all handled in the Rust process; raw key bytes are passed over IPC.
- **Transparent, decoration-free window** with Windows Mica effect for a native, frosted-glass look.

---

## 2. Project Structure

```
lockr/
├── src/                        # SvelteKit frontend
│   ├── app.html                # HTML shell (single <div id="svelte">)
│   ├── app.css                 # Global WinUI-inspired design system
│   ├── lib/
│   │   ├── utils.ts            # cn() helper + TypeScript utility types
│   │   └── components/
│   │       ├── titlebar.svelte # Custom frameless titlebar (drag + close)
│   │       ├── encryptModal.svelte  # Dialog component (bits-ui, currently unused in active flow)
│   │       └── ui/
│   │           ├── button/     # shadcn-style button primitives
│   │           └── navigation-menu/  # Navigation primitives
│   └── routes/
│       ├── +layout.ts          # SPA mode config (ssr=false, prerender=true)
│       ├── +layout.svelte      # Root layout: Titlebar + ContextMenu + fade transitions
│       ├── +page.svelte        # Home: Encrypt / Decrypt entry buttons
│       ├── encrypt/
│       │   ├── +page.svelte    # Encrypt hub: File or Folder choice
│       │   └── file/
│       │       └── +page.svelte  # Full encrypt-file workflow
│       └── decrypt/
│           ├── +page.svelte    # Decrypt hub: File or Folder choice
│           └── file/
│               └── +page.svelte  # Full decrypt-file workflow
│
├── src-tauri/                  # Rust backend
│   ├── Cargo.toml              # Rust dependencies & crate config
│   ├── tauri.conf.json         # Window, build, bundle settings
│   ├── build.rs                # Tauri build script (code-gen hooks)
│   ├── capabilities/           # Tauri v2 permission manifests
│   └── src/
│       ├── main.rs             # Binary entry point → calls lockr_lib::run()
│       ├── lib.rs              # All Tauri commands + crypto logic
│       └── encrypt.rs          # (currently empty — reserved module)
│
├── package.json                # npm deps: @tauri-apps/api, flowbite-svelte, bits-ui, gsap…
├── svelte.config.js            # adapter-static, alias $lib
├── vite.config.js              # @sveltejs/vite-plugin-svelte + @tauri-apps/cli vite plugin
├── tsconfig.json               # TypeScript config
└── pnpm-workspace.yaml         # pnpm workspace (lockr package)
```

---

## 3. Backend — `src-tauri/`

### 3.1 Entry Points — `main.rs` & `lib.rs`

**`main.rs`** (9 lines)

- Declares the `encrypt` module (`mod encrypt;` — currently empty, reserved).
- Suppresses the extra console window on Windows release builds via `#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]`.
- Calls `lockr_lib::run()`, which lives in `lib.rs`.

**`lib.rs` — `pub fn run()`**

This is the Tauri application entry point. It:

1. Calls `tauri::Builder::default()`.
2. In the `.setup()` hook, retrieves the `"main"` webview window and calls `apply_mica(&window, None)` via the `window-vibrancy` crate — **only compiled on Windows** (`#[cfg(target_os = "windows")]`). This applies the Windows 11 Mica material to the window background.
3. Registers `tauri_plugin_opener` (used by the layout's `openUrl` calls).
4. Registers all Tauri commands via `tauri::generate_handler![]` (see §3.3).
5. Runs the event loop.

---

### 3.2 Data Structures

All structs in `lib.rs` are used for Tauri IPC serialization/deserialization.

#### `FileDialogData` — returned by `open_file_dialog`
```rust
#[derive(Serialize)]
struct FileDialogData {
    filepath: PathBuf,  // full absolute path to the selected file
    filename: String,   // just the file name component (e.g., "report.pdf")
}
```
Serialized as JSON and received on the frontend as `{ filepath: string, filename: string }`.

#### `SavingDialogData` — parameter for `open_saving_prompt` (unused currently in frontend)
```rust
#[derive(Deserialize, Debug)]
struct SavingDialogData {
    window_title: String,
    file_extension: String,
    file_type_name: String,
}
```

#### `EncryptionCommandRequestPackage` — parameter for both `final_encryption` and `final_decryption`
```rust
#[derive(Deserialize, Debug)]
struct EncryptionCommandRequestPackage {
    resultant_dir: PathBuf,  // output directory (desktop)
    password: [u8; 32],      // raw 32-byte AES-256 key
    resultname: PathBuf,     // desired output filename (same as source filename)
    filepath: PathBuf,       // input file absolute path
    checked: bool,           // whether to delete the source file after operation
}
```
The `password` field is a fixed-size 32-byte array; Tauri's Serde layer will deserialize a JSON number array into this.

---

### 3.3 Tauri Commands (IPC API)

All functions decorated with `#[tauri::command]` are callable from the frontend via `invoke("command_name", args)`.

---

#### `open_file_dialog() -> Result<FileDialogData, String>`

Opens a native OS file picker dialog (via `rfd::FileDialog`). Returns the selected file's full path and filename. If the dialog is cancelled it returns an `Err("FileDialog does not exist")`. If the path string is empty it recursively retries.

**Called from:** `/encrypt/file` and `/decrypt/file` pages.

---

#### `generate_key() -> [u8; 32]`

Generates a cryptographically secure 32-byte random key using `rand::thread_rng().fill_bytes()`. Returns the raw bytes directly as a JSON number array (e.g., `[187, 43, 201, ...]`).

**Called from:** `/encrypt/file` — after user clicks "Start Encryption", before the key confirmation modal.

---

#### `get_resulting_dir() -> PathBuf`

Returns the current user's Desktop path via `dirs::desktop_dir()`. This is where all encrypted/decrypted output files are saved.

**Called from:** both `/encrypt/file` and `/decrypt/file` after the user confirms the key modal.

---

#### `final_encryption(responsepackage: EncryptionCommandRequestPackage) -> Result<(), String>`

The core encryption command. Full pipeline:

1. Generates a random 12-byte **nonce** (`rand::thread_rng().fill_bytes()`).
2. Constructs an `Aes256Gcm` cipher from the provided 32-byte key.
3. Builds the output path: joins `resultant_dir + resultname`, then appends `.aes` extension using `path.add_extension("aes")`.
4. Creates output file and opens input file; wraps both in `BufReader`/`BufWriter`.
5. **Writes the 12-byte nonce as the first bytes of the output file** — this is essential for decryption.
6. Reads the source file in **4 KB chunks** (`BUFFER_SIZE = 4`, buffer = 4096 bytes).
7. For each chunk, calls `cipher.encrypt(nonce, &buffer[..n])` — each chunk is independently authenticated+encrypted using the same nonce.
8. Writes each encrypted chunk (plaintext_len + 16 bytes GCM tag) to the output file.
9. Returns `Ok(())` on success or an error string on any I/O or crypto failure.

**Output file format:**
```
[12 bytes: nonce] [chunk1_ciphertext + 16-byte tag] [chunk2_ciphertext + 16-byte tag] ...
```

**Called from:** `/encrypt/file`.

---

#### `final_decryption(responsepackage: EncryptionCommandRequestPackage) -> Result<(), String>`

The mirror of `final_encryption`. Pipeline:

1. Constructs an `Aes256Gcm` cipher from the provided 32-byte key.
2. Builds output path: joins `resultant_dir + resultname`, then **strips** the extension (removing `.aes`) to recover the original filename.
3. Opens the encrypted file; wraps in `BufReader`.
4. **Reads exactly the first 12 bytes as the nonce** using `read_exact`.
5. Reads the rest of the file in **4112-byte chunks** (`4096 + 16` — matching exactly what encryption wrote: 4096 plaintext + 16-byte GCM tag).
6. Uses a precise fill-loop to guarantee the full 4112 bytes are read per chunk (preventing partial reads that would break the GCM tag boundary).
7. For each chunk, calls `cipher.decrypt(nonce, &buffer[..n])` and writes the resulting plaintext.
8. Returns `Ok(())` or an error.

> **Critical design note:** chunk sizes must match exactly between encryption and decryption because AES-GCM authenticates entire chunks. The decryption buffer is `4096 + 16 = 4112` bytes to absorb the tag that was appended by encryption.

**Called from:** `/decrypt/file`.

---

#### `read_nounce_bytes(path: PathBuf) -> Result<[u8; 12], String>`

Opens a file and reads exactly the first 12 bytes (the nonce) without consuming the rest of it. This command exists so the frontend can **pre-fetch and display** nonce data if needed. In the current flow it is called on the decryption page before constructing the decryption package — the nonce is read, stored in a frontend variable `nounce`, but is **not actually passed** to `final_decryption` (the backend re-reads it itself from the file).

**Called from:** `/decrypt/file`.

---

#### `open_folder_dialog() -> String`

Opens a native folder picker and returns the selected directory path as a string. Error handling is minimal (a `TODO` note in the source acknowledges this). Not currently wired to any active UI route (folder encrypt/decrypt routes are stubs).

---

#### `open_in_explorer(new_file_path: String) -> Result<(), String>`

Launches Windows Explorer and selects/highlights the specified file using:
```
explorer.exe /select, <path>
```
Uses `std::process::Command` to spawn the process. Works only on Windows.

**Called from:** both `/encrypt/file` and `/decrypt/file` success screens — "Show File Location" button.

---

#### `get_file_path()` (async, no return value)

An older/prototype command. Opens a file dialog and immediately calls the internal `encript_file_by_path()` helper function. Not called from any current UI page. Exists as legacy prototype code.

---

#### `open_saving_prompt(output_case_type: String) -> Result<String, String>` (not registered in handler)

Opens a "Save As" dialog. Accepts either `"type_encryption_save"` or `"type_decryption_save"` to customise the dialog title/filter. Returns the selected save path. **Not currently registered** in `generate_handler![]` and not connected to any UI. Likely planned for future use.

---

#### Internal: `encript_file_by_path(path_to_file: PathBuf)` (private, not a command)

An older prototype function, not exposed as a Tauri command. Reads the entire file into memory, encrypts it in one shot (not buffered), prepends the nonce, and writes to a `.bin` file. Superseded by the chunked `final_encryption`. Retained in the codebase as reference.

---

### 3.4 Cryptographic Engine

| Property | Value |
|----------|-------|
| Algorithm | AES-256-GCM (Authenticated Encryption with Associated Data) |
| Key size | 256 bits (32 bytes) |
| Nonce size | 96 bits (12 bytes), randomly generated per-file |
| Authentication tag | 128 bits (16 bytes), appended to each chunk |
| Chunk size (encryption) | 4096 bytes |
| Chunk size (decryption) | 4112 bytes (4096 + 16-byte tag) |
| Key source | `rand::thread_rng` (OS CSPRNG on all platforms) |
| Nonce source | `rand::thread_rng` (OS CSPRNG on all platforms) |
| Crate | `aes-gcm = "0.10.3"` |

**Reuse of nonce across chunks:** The current implementation uses the same nonce for every chunk within a single file. This is a known limitation — AES-GCM is not safe for nonce reuse across different plaintexts, but since each file gets a unique random nonce, cross-file reuse is avoided. Intra-file chunk reuse is an accepted trade-off in this prototype.

---

### 3.5 Dependencies (`Cargo.toml`)

| Crate | Purpose |
|-------|---------|
| `tauri = "2"` | Core framework: window management, IPC, packaging |
| `tauri-plugin-opener = "2"` | `openUrl()` from the frontend shell |
| `serde` + `serde_json` | Serialize/Deserialize for IPC data structs |
| `rfd = "0.15.4"` | Native OS file/folder picker dialogs |
| `aes-gcm = "0.10.3"` | AES-256-GCM authenticated encryption |
| `rand = "0.8.5"` | CSPRNG for key and nonce generation |
| `window-vibrancy = "0.7.1"` | Windows 11 Mica / Acrylic background effect |
| `dirs = "6.0.0"` | Cross-platform user directory paths (Desktop, Home…) |
| `tokio = "1.51.0"` | Async runtime (available but Tauri handles its own) |

---

### 3.6 Build & Configuration (`tauri.conf.json`)

| Setting | Value | Explanation |
|---------|-------|-------------|
| `productName` | `lockr` | Display name |
| `identifier` | `com.sharmadevanshu089.lockr` | Reverse-DNS bundle ID |
| `devUrl` | `http://localhost:1420` | Vite dev server |
| `beforeDevCommand` | `pnpm dev` | Starts Vite when running `tauri dev` |
| `frontendDist` | `../build` | Built static assets for production |
| `width` × `height` | 800 × 600 | Window dimensions |
| `minWidth` × `minHeight` | 799 × 599 | Minimum resize guard |
| `decorations` | `false` | No OS titlebar — uses custom Svelte titlebar |
| `transparent` | `true` | Window allows transparency for Mica material |
| `csp` | `null` | No Content Security Policy (dev convenience) |

---

## 4. Frontend — `src/`

### 4.1 Static Config (`+layout.ts`)

```ts
export const ssr = false;
export const prerender = true;
```

- **`ssr = false`:** Disables server-side rendering — required because Tauri serves a static bundle with no server.
- **`prerender = true`:** Causes SvelteKit to pre-render all routes at build time into static HTML files, which Tauri then packages.

---

### 4.2 Global Styles (`app.css`)

The design language is a **WinUI 3 / Windows 11 Fluent Design** inspired system, implemented in plain CSS. Key design tokens and components:

#### CSS Variables (Design Tokens)

| Token | Value | Usage |
|-------|-------|-------|
| `--font-sans` | Segoe Fluent / Segoe UI Variable / system-ui | All text |
| `--text-primary` | `#E6E7E8` | Main text color |
| `--text-secondary` | `#5582bd` (blue-grey) | Secondary / accent text; `.blue` class |
| `--text-disabled` | `#6F757B` | Disabled states |
| `--accent-color` | `#0078D4` | WinUI accent blue (buttons, focus rings, checkbox) |
| `--control-fill-color-default` | `rgba(255,255,255,0.06)` | Subtle translucent fill |
| `--control-fill-color-secondary` | `rgba(255,255,255,0.12)` | Hover fill |
| `--dialog-bg` | `rgba(32,32,32,0.95)` | Dialog background |
| `--dialog-accent` | `#0078D4` | Dialog confirm button |

#### Component Classes

| Class | Element | Behaviour |
|-------|---------|-----------|
| `.winui-button` | `<button>` | Translucent fill, hover brightens, focus ring, disabled fades to 45% opacity |
| `.winui-filebox` | File picker row | Translucent container, text field + blue "Browse" button |
| `.winui-checkbox` | Checkbox + label | Custom checkbox with checkmark glyph, accent fill when checked |
| `.winui-choice` | Choice chip | Hover brightens, selected state uses blue tint |
| `.dialog-overlay` | Fixed fullscreen dim | Semi-black overlay behind dialog |
| `.dialog` | Dialog box | Dark translucent panel, rounded, shadow |
| `.dialog-button` | Confirm button | Accent blue, hover lightens |
| `.dialog-input` | Text input in dialog | Translucent, blue focus ring |
| `.herobox` | Button container | Flex row, 40 vh height — centers action buttons vertically |
| `.blue` | `<span>` | Applies `--text-secondary` (blue-grey highlight in headings) |

**Typography:** `h1`/`h2`/`h3` have `padding-top: 20vh` — this pushes them down from the titlebar, creating generous vertical whitespace.

**Scrollbars:** Hidden globally via `::-webkit-scrollbar { display: none }` and `scrollbar-width: none`.

> **Note:** The CSS file contains a large block of commented-out Oklch/dark-mode theme variables — this was an in-progress experiment with a more complete Shadcn-style token system. It was abandoned in favour of the current flat WinUI approach.

---

### 4.3 Utility Module (`lib/utils.ts`)

```ts
export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}
```

Standard `shadcn/ui`-style class merge helper — combines `clsx` (conditional classes) and `tailwind-merge` (deduplicates conflicting Tailwind classes). Used by the auto-generated `ui/button` and `ui/navigation-menu` components.

Also exports utility TypeScript types:
- `WithoutChild<T>` / `WithoutChildren<T>` — strips the Svelte 5 `child`/`children` prop from a type (used for primitive component APIs).
- `WithoutChildrenOrChild<T>` — both combined.
- `WithElementRef<T, U>` — adds an optional `ref` prop (for DOM element binding).

---

### 4.4 Shared Components (`lib/components/`)

#### `titlebar.svelte`

A **custom window titlebar** built on `bits-ui`'s `Toolbar` primitive, needed because the OS window decorations are disabled (`decorations: false` in `tauri.conf.json`).

**Structure:**
- **Left zone:** Conditionally renders a back arrow button (currently commented out — code exists but is disabled).
- **Centre:** "Lockr" text logo, absolutely positioned to stay centred regardless of side buttons.
- **Right zone:** A close button (`X` icon). On hover it turns red with a glow effect.

**Key attributes:**
- `data-tauri-drag-region` — tells Tauri that this element acts as the window drag handle. The entire toolbar (and all its children with this attribute) can be used to drag the window.
- `getCurrentWindow().close()` — calls the Tauri API to close the app window.
- The back button navigates to `/` using SvelteKit's `goto('/')`.

#### `encryptModal.svelte`

A `bits-ui` Dialog-based modal with animated overlay and content. It accepts:
- `open: boolean` — externally controlled visibility.
- `onClose: () => void` — callback for when the modal is dismissed.

Contains a file path input and Encrypt/Cancel buttons. **Currently not used in the active encryption flow** — the active pages implement their own inline dialog overlay using `.dialog-overlay`/`.dialog` CSS classes. This component exists as a more polished alternative, likely to be integrated later.

---

## 5. Routes & Pages

### 5.1 `/` — Home (`+page.svelte`)

**Purpose:** Application entry point. Presents the two primary actions.

**Imports:** `Lock`, `Key` icons (lucide-svelte), `goto` (SvelteKit navigation), `Breadcrumb`, `Heading`, `GradientButton` (flowbite-svelte), `gsap` (imported but not used in this page).

**UI:**
- Breadcrumb: `Home`
- Heading: "Encrypt your **files and folders.**"
- Two `.winui-button` buttons: **Encrypt** and **Decrypt**.

**Logic:**
- `handleEncryptClick()` → `goto('/encrypt')`
- `handleDecryptClick()` → `goto('/decrypt')`

**State variables:** None (purely navigational).

---

### 5.2 `/encrypt` — Encrypt Hub

**Purpose:** Secondary choice: encrypt a **File** or a **Folder**.

**UI:**
- Breadcrumb: `Home > Encrypt`
- Heading: "Select between **files** or **folders.**"
- Two `.winui-button` buttons: **File** and **Folder**.

**Logic:**
- `handleFileClick()` → `goto('/encrypt/file')`
- `handleFolderClick()` → `goto('/encrypt/folder')` — **route does not exist yet** (planned).

---

### 5.3 `/encrypt/file` — Encrypt File Workflow

The most feature-complete page. Manages the full multi-step encryption user journey.

#### State Variables

| Variable | Type | Initial Value | Purpose |
|----------|------|---------------|---------|
| `filePath` | `string` | `"File Not Selected"` | Path shown in the read-only text field |
| `fileData` | `any` | `undefined` | Raw response from `open_file_dialog` IPC call |
| `filename` | `string` | `undefined` | Extracted filename from `fileData` |
| `checked` | `boolean` | `false` | Whether to delete original file after encryption |
| `browseButton` | `HTMLElement` | (DOM ref) | Reference to "Browse" text node, toggled by spinner |
| `SpinnerInBrowseButton` | `HTMLElement` | (DOM ref) | Flowbite Spinner inside Browse button |
| `heroButton` | `HTMLElement` | (DOM ref) | "Start Encryption" button — disabled until file selected |
| `initialMenu` | `boolean` | `true` | Controls visibility of the file picker UI |
| `modalSelect` | `boolean` | `false` | Controls visibility of the key confirmation dialog |
| `password` | `string` | `undefined` | String representation of the 32-byte key |
| `disabledModalConfirm` | `boolean` | `true` | Disables the "OK" button until key is generated |
| `passwordArray` | `[u8,32]` | `undefined` | Raw number array — the actual 32-byte key |
| `loading` | `boolean` | `false` | Shows the loading spinner during crypto operation |
| `loaderState` | `string` | `"Initializing..."` | Text label shown under the spinner |
| `desktopDirectory` | `string` | `undefined` | Desktop path (from `get_resulting_dir`) |
| `sucessState` | `boolean` | `false` | Shows success screen after encryption completes |

#### Lifecycle

`onMount` acquires three DOM element references by ID (`HeroButton`, `SpinnerInBrowse`, `BrowseText`) so they can be shown/hidden imperatively during async operations.

#### Functions

**`openFileDialog()`**
1. Shows the spinner, hides the "Browse" text label (visual feedback that dialog is opening).
2. `invoke("open_file_dialog")` → gets `{ filepath, filename }`.
3. Stores both in `fileData`, `filename`, `filePath`.
4. Hides spinner, shows "Browse" text again.
5. Enables the `heroButton` (`disabled = false`).
6. On error: restores spinner/button state, logs error.

**`initiateEncryption()`**
1. Hides the file picker UI: `initialMenu = false`.
2. Shows the key dialog: `modalSelect = true`.
3. `invoke("generate_key")` → receives `[u8; 32]` as a JS number array.
4. Converts to comma-separated string for display: `passwordArray.toString()`.
5. Enables the "OK" button: `disabledModalConfirm = false`.

**`confirmEncryptionModal()`**
1. Hides the key dialog: `modalSelect = false`.
2. Shows loading spinner: `loading = true`, `loaderState = "Encrypting"`.
3. `invoke("get_resulting_dir")` → `desktopDirectory`.
4. Builds the `encryptionPackageJS` object matching `EncryptionCommandRequestPackage`.
5. `invoke("final_encryption", { responsepackage: encryptionPackageJS })`.
6. On success: `sucessState = true`.
7. On error: logs, but **no user-facing error UI** (marked TODO).
8. `loading = false`.

**`showFileLocation()`**
1. Derives the expected output filename: replaces the file extension with `.aes` using a regex (`/\.[^.]+$/`).
2. Constructs the full path: `desktopDirectory + "\\" + newFilename`.
3. `invoke("open_in_explorer", { newFilePath: new_file_path })` → opens Windows Explorer highlighting the file.

---

### 5.4 `/decrypt` — Decrypt Hub

**Purpose:** Secondary choice: decrypt a **File** or a **Folder** (mirror of `/encrypt`).

**UI:**
- Breadcrumb: `Home > Decrypt`
- Heading: "Select between **files** or **folders.**"
- **File** → `/decrypt/file`, **Folder** → `/decrypt/folder` (stub).

---

### 5.5 `/decrypt/file` — Decrypt File Workflow

Mirror of `/encrypt/file` but for decryption. Key differences:

#### Additional State Variable

| Variable | Type | Initial Value | Purpose |
|----------|------|---------------|---------|
| `nounce` | `[u8,12]` | `undefined` | The 12-byte nonce read from the encrypted file's header |

#### Key Differences vs Encrypt Flow

1. **No key generation.** The modal asks the user to *enter* the key that was used during encryption. `password` is a user-typed string, not auto-generated.
2. **`disabledModalConfirm` starts `false`** after `initiateDecryption()` — the modal OK button is immediately enabled because key input is manual.
3. **Key parsing in `confirmDecryptionModal()`:** The user's input string (expected to be a comma-separated list of numbers like `"187,43,201,..."`) is split by comma and mapped to numbers: `stringArray.map(Number)`.
4. **Nonce pre-read:** Calls `invoke("read_nounce_bytes", { path: filePath })` before sending the decryption package. The result is stored locally (`nounce`) but is not forwarded to `final_decryption` — the backend reads it again internally.
5. **Output filename:** Strips `.aes` extension from the filename. `filename.endsWith('.aes')` → `filename.slice(0, -4)`.
6. **No "delete original" checkbox.** The encrypted file is kept after decryption.

---

## 6. UI State Machines

### 6.1 Encrypt File — State Machine

```
┌─────────────────────────────────────────────────────────────┐
│                    STATE: initialMenu = true                 │
│  (file picker visible, heroButton disabled)                 │
│                                                             │
│  User clicks "Browse" ──► openFileDialog()                  │
│    Browse spinner shows   ──► IPC: open_file_dialog         │
│    File selected           ──► filePath, filename set       │
│    HeroButton enabled      ◄── spinner hides                │
│                                                             │
│  User checks/unchecks "Also Delete original"  (checked var) │
│                                                             │
│  User clicks "Start Encryption" ──► initiateEncryption()   │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│              STATE: modalSelect = true                       │
│  (key dialog visible)                                       │
│                                                             │
│  IPC: generate_key ──► passwordArray (32 bytes)            │
│  password = passwordArray.toString() (displayed in input)   │
│  User may edit key manually                                 │
│  OK button enabled                                          │
│                                                             │
│  User clicks "OK" ──► confirmEncryptionModal()             │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│              STATE: loading = true                           │
│  (spinner + "Encrypting" label visible)                     │
│                                                             │
│  IPC: get_resulting_dir ──► desktopDirectory               │
│  IPC: final_encryption ──► encryption runs in Rust         │
└──────┬──────────────────────────────────────────────────────┘
       │ success                  │ error
       ▼                          ▼
┌─────────────────┐    ┌────────────────────────────────────┐
│ STATE: sucessState│   │ loading = false, error logged      │
│  = true          │   │ (no user-facing error UI — TODO)   │
│                 │   └────────────────────────────────────┘
│ "Successfully   │
│  encrypted"     │
│ [Show File Loc] │──► IPC: open_in_explorer
│ [Home] link     │──► goto('/')
└─────────────────┘
```

---

### 6.2 Decrypt File — State Machine

```
┌──────────────────────────────────────────────────────────────┐
│                  STATE: initialMenu = true                    │
│  (file picker visible, heroButton disabled)                   │
│                                                              │
│  User clicks "Browse" ──► openFileDialog()                   │
│    IPC: open_file_dialog ──► filePath, filename             │
│    HeroButton enabled                                        │
│                                                              │
│  User clicks "Start Decryption" ──► initiateDecryption()    │
└────────────────────────┬─────────────────────────────────────┘
                         │
                         ▼
┌──────────────────────────────────────────────────────────────┐
│              STATE: modalSelect = true                        │
│  (key input dialog visible)                                  │
│                                                              │
│  User types their 32-byte key string                        │
│  (format: "187,43,201,...,99" — comma-separated numbers)    │
│  OK button immediately enabled                               │
│                                                              │
│  User clicks "OK" ──► confirmDecryptionModal()              │
└────────────────────────┬─────────────────────────────────────┘
                         │
                         ▼
┌──────────────────────────────────────────────────────────────┐
│         STATE: loading = true, loaderState = "Reading..."    │
│                                                              │
│  password.split(",").map(Number) ──► passwordArray          │
│  IPC: read_nounce_bytes ──► nounce (stored, not forwarded)  │
│  IPC: get_resulting_dir ──► desktopDirectory               │
│  loaderState = "Decrypting"                                 │
│  IPC: final_decryption ──► decryption runs in Rust         │
└──────┬────────────────────────────────────────────────────────┘
       │ success                  │ error
       ▼                          ▼
┌─────────────────┐    ┌────────────────────────────────────┐
│ STATE: sucessState│   │ loading = false, error logged       │
│  = true          │   │ (no user-facing error UI — TODO)   │
│                 │   └────────────────────────────────────┘
│ "Successfully   │
│  decrypted"     │
│ [Show File Loc] │──► IPC: open_in_explorer
│ [Home] link     │──► goto('/')
└─────────────────┘
```

---

## 7. IPC Data Flow — End-to-End

### 7.1 Encryption Flow

```
Frontend                                    Rust Backend
────────────────────────────────────────────────────────────
invoke("open_file_dialog")              ──► opens rfd dialog
                                        ◄── { filepath, filename }

invoke("generate_key")                  ──► rand::fill_bytes([u8;32])
                                        ◄── [187, 43, 201, ..., 99]  (32 numbers)

password = passwordArray.toString()     (local: "187,43,201,...,99")

invoke("get_resulting_dir")             ──► dirs::desktop_dir()
                                        ◄── "C:\\Users\\sharm\\Desktop"

invoke("final_encryption", {            ──► Rust receives EncryptionCommandRequestPackage
  responsepackage: {                        - generates nonce
    resultant_dir: "C:\\...\\Desktop",      - opens source file BufReader
    password: [187,43,...],                 - creates output.aes BufWriter
    resultname: "document.pdf",            - writes 12-byte nonce header
    filepath: "C:\\...\\document.pdf",     - loop: read 4KB → encrypt → write
    checked: false                         - returns Ok(())
  }
})
                                        ◄── Ok(())   (void)

invoke("open_in_explorer", {            ──► explorer.exe /select, path
  newFilePath: "C:\\...\\document.aes"
})
```

### 7.2 Decryption Flow

```
Frontend                                    Rust Backend
────────────────────────────────────────────────────────────
invoke("open_file_dialog")              ──► opens rfd dialog (.aes file expected)
                                        ◄── { filepath, filename: "document.aes" }

[user types/pastes key string]
password = "187,43,201,...,99"

passwordArray = password.split(",").map(Number)   (local parsing)

invoke("read_nounce_bytes", {           ──► opens file, reads first 12 bytes
  path: "C:\\...\\document.aes"        ◄── [u8;12]  (nonce — stored but not used further)
})

invoke("get_resulting_dir")             ──► dirs::desktop_dir()
                                        ◄── "C:\\Users\\sharm\\Desktop"

invoke("final_decryption", {            ──► Rust receives EncryptionCommandRequestPackage
  responsepackage: {                        - reconstructs key from password bytes
    resultant_dir: "C:\\...\\Desktop",      - opens source file BufReader
    password: [187,43,...],                 - strips .aes → output path "document.pdf"
    resultname: "document.aes",            - reads 12-byte nonce from file header
    filepath: "C:\\...\\document.aes",     - loop: read 4112B → decrypt → write plaintext
    checked: false                         - returns Ok(())
  }
})
                                        ◄── Ok(())

invoke("open_in_explorer", {            ──► explorer.exe /select, path
  newFilePath: "C:\\...\\document.pdf"
})
```

---

## 8. Cryptographic Design

### File Format (Encrypted `.aes` file)

```
Offset    Size      Content
─────────────────────────────────────────────────────
0         12 bytes  AES-GCM Nonce (random, unique per file)
12        n bytes   Chunk 1 ciphertext (4096 plaintext → 4112 with 16-byte tag)
12+4112   n bytes   Chunk 2 ciphertext
...       ...       ...
EOF               Last chunk ciphertext (variable size)
```

### Key Management

- During **encryption:** key is generated in Rust, sent to the frontend as a raw byte array, displayed to the user as a comma-separated string (e.g., `"187,43,201,…"`). The user _must_ copy this key; it is not stored anywhere.
- During **decryption:** the user must paste the comma-separated key string into the dialog. The frontend parses it back to a number array and sends it to Rust.
- **No key storage, no key derivation, no passphrase hashing.** The raw bytes _are_ the key. This is intentional simplicity for a prototype — KDF (e.g., Argon2 or PBKDF2) would be needed for a production security model.

### Security Properties

| Property | Status |
|----------|--------|
| Confidentiality | ✅ AES-256 ensures ciphertext reveals no plaintext info |
| Integrity / Tamper detection | ✅ GCM authentication tag — any modification causes decrypt to fail |
| Unique nonce per file | ✅ Generated fresh via CSPRNG |
| Nonce reuse across chunks | ⚠️ Same nonce used within one file (see §3.4 note) |
| Key storage | ❌ No storage — user manually copies/pastes |
| Key stretching | ❌ No KDF — raw bytes used directly |
| Delete-on-encrypt | ⚠️ `checked` field parsed but the delete logic is not wired in the command |

---

## 9. Window & Visual System

The application uses a **decoration-free, transparent window** to achieve a native Windows 11 aesthetic with the Mica material (a frosted-glass effect that blends with the desktop wallpaper).

**How it works:**
1. `tauri.conf.json` sets `decorations: false` and `transparent: true`.
2. In `lib.rs`, `apply_mica(&window, None)` is called on Windows via the `window-vibrancy` crate. On other OSes this block is skipped (`#[cfg(target_os = "windows")]`).
3. The Svelte frontend's `app.css` uses `rgba(...)` backgrounds with low alpha (e.g., `rgba(255,255,255,0.06)`) so the Mica texture shows through controls.
4. The root layout wraps the content in `bg-background` (Tailwind class) but since no opaque theme is set, the background remains transparent.

**Custom Titlebar (`titlebar.svelte`):**
- Uses `data-tauri-drag-region` attributes on the toolbar and its children, allowing the user to drag the window from anywhere on the titlebar.
- The `getCurrentWindow().close()` call closes the window programmatically.
- Page transitions use Svelte's `fade` transition (150ms) keyed by `$page.url.pathname`, so every route change fades out the old content and fades in the new.

**Right-click Context Menu (`+layout.svelte`):**
- A `bits-ui` `ContextMenu` wraps the entire app content area.
- Right-clicking anywhere on the app shows a context menu with: **Issues** (opens GitHub issues), **Source** (opens GitHub repo), **About Me** (opens developer's GitHub profile). Uses `openUrl` from `@tauri-apps/plugin-opener`.

---

## 10. Debug Logging System

Both the Rust backend and each Svelte page maintain a `const DEBUG = true` flag. When enabled:

**Rust (stdout via `println!`):**
- Every function logs its name on entry: `"<function> is loading"`.
- Every significant variable logs its current value upon assignment.
- Return values are logged before returning.

**Frontend (browser console via `console.log`):**
- Same pattern — function entry, variable values, IPC calls before `await`, IPC returns after `await`.

To disable: set `const DEBUG = false` in each file (frontend) or `const DEBUG: bool = false;` in `lib.rs`. All logging is gated behind `if (DEBUG) { ... }` / `if DEBUG { ... }` blocks.

---

## 11. Known Limitations & TODOs

| # | Area | Description |
|---|------|-------------|
| 1 | **Folder support** | `/encrypt/folder` and `/decrypt/folder` routes do not exist. The hub pages link to them but will 404. |
| 2 | **Error handling (frontend)** | All `catch` blocks in the file pages log errors to the console but show no user-facing error message. Marked `// TODO` in source. |
| 3 | **Delete-on-encrypt** | The `checked` boolean is passed to Rust's `EncryptionCommandRequestPackage` but the Rust command does not act on it — `fs::remove_file` is commented out. |
| 4 | **Key UX** | The key is displayed as a raw comma-separated byte string. A hex or base64 encoding would be shorter and less error-prone for users to copy/paste. |
| 5 | **Nonce reuse** | The same nonce is used for all chunks within one file. For a prototype this is acceptable, but production code should use a nonce counter or per-chunk nonces. |
| 6 | **Key storage** | No KDF (Key Derivation Function) — the raw bytes are the key. No passphrase-protected keyfile. |
| 7 | **`encryptModal.svelte`** | The polished Dialog component exists but is not connected to any active route. |
| 8 | **`open_saving_prompt`** | Implemented in Rust but not registered in the `generate_handler![]` and not called from any UI. |
| 9 | **`get_file_path` / `encript_file_by_path`** | Legacy prototype commands — not connected to any UI, kept as dead code. |
| 10 | **Back button** | Code for a back-navigation arrow in the titlebar exists but is commented out. |
| 11 | **GSAP animations** | `gsap` is imported on several pages but no animations have been implemented yet. |
| 12 | **`open_folder_dialog` error handling** | The function uses `.unwrap()` with a TODO comment — panics if the user cancels the dialog. |
