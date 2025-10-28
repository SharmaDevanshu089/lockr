# 🔒 Lockr: Secure, Performant, & Beautiful File Encryption

Lockr is a next-generation, high-performance file encryption utility designed for top-level security, speed, and a seamless user experience. It merges the rock-solid performance of Rust for its memory-safe backend with Tauri’s lightweight, secure, cross-platform desktop capabilities. The modern SvelteKit frontend offers a reactive, glassmorphic UI that makes encryption a delight, not a chore.

![Lockr UI Screenshot](<video.gif>)

---

## 💡 Why Lockr?

Most security tools force users to pick between power and aesthetics. Lockr is crafted to solve this—it’s proof that security, speed, and beauty can coexist. Inspired by "write less, do more" principles, the UI is as enjoyable as it is robust, making cryptographic operations intuitive and visually engaging.

---

## 🎨 Design Philosophy

- **Glassmorphism & Depth**  
  “Frosted glass” effects (backdrop-blur, bg-white/10) establish clear visual hierarchy and interactive depth for cards and modals.

- **Living Interface**  
  Dynamic mouse-tracking spotlight and subtle micro-interactions make Lockr feel alive and interactive.

- **Subtle Feedback**  
  Hovering on UI elements gently scales cards/icons, adds a colored glow, and subtly fades others, making user actions feel intentional.

- **Performance First**  
  Using Svelte and direct CSS transitions, animations remain buttery-smooth off the main thread, ensuring a snappy, responsive feel.

---

## ✨ Features

- **Blazing-Fast Performance**  
  Rust powers all cryptography for maximum security without bottlenecks.

- **In-Memory Security**  
  All encryption & decryption occurs in-memory; unencrypted data is never written to disk.

- **Tiny & Cross-Platform**  
  Leveraging Tauri, Lockr runs natively on Windows, macOS, and Linux with minimal footprint.

- **Glassmorphic UI**  
  SvelteKit & Tailwind CSS create a stunning, responsive, glassy UI.

- **File & Folder Encryption**  
  Encrypts both single files and folders (folders are zipped-on-the-fly).

- **Async-First**  
  The frontend speaks to the Rust backend asynchronously so the UI never freezes—even during heavy tasks.

---

## 🛠 Tech Stack

| Layer          | Tech             | Why?                                                                 |
|----------------|------------------|----------------------------------------------------------------------|
| **Backend**    | Rust             | Memory safety, zero-cost abstractions, best-in-class multi-threading |
| **Framework**  | Tauri            | Secure, lightweight, instant startup, true native integration        |
| **Frontend**   | SvelteKit        | No virtual DOM, truly reactive, write less/do more, fast updates     |
| **UI Styling** | Tailwind CSS     | Utility-first, expressive, customizable glassmorphic designs         |
| **UI Lib**     | Bits UI (Svelte) | Headless, accessible, fully customizable components                  |

---

## 🚀 Development Journey

Lockr is under **active development**. Core UI is complete, several features are shipped, and more are underway!

**Current Progress:**

- [x] Main selection screen with glassmorphism and spotlight.
- [x] Svelte component logic for file/folder choices.
- [x] Custom, cross-platform titlebar.
- [x] Animations, hover effects, transitions.
- [x] Custom context menu.
- [x] GitHub Actions CI/CD pipeline for releases.
- [x] Application flow diagram.
- [ ] Tauri commands: get_folder_path, encrypt_file, decrypt_file.
- [ ] Rust backend: crypto crate, password handling, folder-to-zip logic.
- [ ] UI modals: password input, success/error notifications.

**Velocity:**  
After a tech-stack pivot (React → Svelte), a full-featured custom UI and robust architecture fell in place within the first week.

---

## 📦 Getting Started

**Prerequisites:**  
- [Tauri prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites): Rust, Node.js, OS dependencies  
- `pnpm` package manager

**Clone & Setup:**

```
git clone https://github.com/SharmaDevanshu089/lockr.git
cd lockr
pnpm install
npm run tauri dev
```

---

## 📄 License

Lockr is open source under the MIT License.

---

**Made with ❤️ by [SharmaDevanshu089](https://github.com/SharmaDevanshu089)**

---

Enjoy a seamless blend of security, speed, and UI beauty with Lockr!
