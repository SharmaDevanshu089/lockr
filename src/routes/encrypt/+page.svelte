<!-- just comment to check workflow -->
<script lang="ts">
  import { FolderLock, FileLock2, X } from "lucide-svelte";
  import { invoke } from '@tauri-apps/api/core';
  import { open } from '@tauri-apps/plugin-dialog';

  let hovered: 'encrypt' | 'decrypt' | null = null;
  let showModal = false;
  let selectedFilePath = '';
  let modalType: 'file' | 'folder' | null = null;

  function handleDecryptClick() {
    console.log("Folder clicked!");
    modalType = 'folder';
    showModal = true;
  }

  function intiateFile() {
    console.log('File clicked!');
    modalType = 'file';
    showModal = true;
  }

  async function browseFile() {
    try {
      const selected = await open({
        multiple: false,
        directory: modalType === 'folder'
      });

      if (selected) {
        selectedFilePath = selected as string;
      }
    } catch (error) {
      console.error('Error selecting file:', error);
    }
  }

  function handleEncrypt() {
    if (!selectedFilePath) return;

    console.log(`Encrypting ${modalType}:`, selectedFilePath);
    invoke('get_file_path', { path: selectedFilePath });

    // Reset modal
    closeModal();
  }

  function closeModal() {
    showModal = false;
    selectedFilePath = '';
    modalType = null;
  }

  function updateSpotlight(e: MouseEvent) {
    const root = document.documentElement;
    root.style.setProperty('--x', `${e.clientX}px`);
    root.style.setProperty('--y', `${e.clientY}px`);
  }
</script>

<svelte:window on:mousemove={updateSpotlight} />

<div class="relative w-full overflow-hidden bg-[#020618] text-white">

  <!-- spotlight overlay -->
  <div
    class="absolute inset-0 transition-all duration-300 pointer-events-none"
    style="background: radial-gradient(400px circle at var(--x, 50%) var(--y, 50%), rgba(100, 200, 255, 0.12), transparent 40%);"
  ></div>

  <!-- card grid -->
  <div class="relative z-10 grid min-h-[calc(100vh-8rem)] grid-cols-1 gap-8 p-10 md:grid-cols-2">
    <!-- Encrypt Panel -->
    <div
      id="encrypt"
      role="button"
      tabindex="0"
      class="group relative flex h-full cursor-pointer items-center justify-center rounded-3xl border border-white/20 bg-white/10 p-10 shadow-2xl backdrop-blur-xl transition-all duration-500 hover:scale-[1.03] hover:border-primary/40"
      on:mouseenter={() => (hovered = 'encrypt')}
      on:mouseleave={() => (hovered = null)}
      on:click={intiateFile}
      on:keydown={(e) => (e.key === 'Enter' || e.key === ' ') && intiateFile()}
    >
      <div class="text-center transition-all duration-500" class:opacity-80={hovered === 'decrypt'}>
        <FileLock2 class="mx-auto h-24 w-24 text-primary transition-all duration-500 group-hover:scale-110" />
        <h2 class="mt-6 text-5xl font-extrabold">File</h2>
        <p class="mt-3 text-lg text-gray-300">Encript a File.</p>
      </div>

      <!-- subtle glow -->
      <div
        class="absolute -inset-0.5 rounded-3xl bg-linear-to-r from-blue-500/20 to-cyan-400/20 opacity-0 blur-xl transition-opacity duration-500 group-hover:opacity-100 pointer-events-none"
      ></div>
    </div>

    <!-- Decrypt Panel -->
    <div
      id="decrypt"
      role="button"
      tabindex="0"
      class="group relative flex h-full cursor-pointer items-center justify-center rounded-3xl border border-white/20 bg-white/10 p-10 shadow-2xl backdrop-blur-xl transition-all duration-500 hover:scale-[1.03] hover:border-primary/40"
      on:mouseenter={() => (hovered = 'decrypt')}
      on:mouseleave={() => (hovered = null)}
      on:click={handleDecryptClick}
      on:keydown={(e) => (e.key === 'Enter' || e.key === ' ') && handleDecryptClick()}
    >
      <div class="text-center transition-all duration-500" class:opacity-80={hovered === 'encrypt'}>
        <FolderLock class="mx-auto h-24 w-24 text-primary transition-all duration-500 group-hover:scale-110" />
        <h2 class="mt-6 text-5xl font-extrabold">Folder</h2>
        <p class="mt-3 text-lg text-gray-300">Encript a Folder.</p>
      </div>

      <div
        class="absolute -inset-0.5 rounded-3xl bg-linear-to-r from-cyan-400/20 to-blue-500/20 opacity-0 blur-xl transition-opacity duration-500 group-hover:opacity-100 pointer-events-none"
      ></div>
    </div>
  </div>

  <!-- Modal -->
  {#if showModal}
    <div
      class="fixed inset-0 z-50 flex items-center justify-center bg-black/70 backdrop-blur-sm"
      on:click={closeModal}
      role="button"
      tabindex="0"
      on:keydown={(e) => e.key === 'Escape' && closeModal()}
    >
      <div
        class="relative w-full max-w-lg rounded-3xl border border-white/20 bg-[#020618]/95 p-8 shadow-2xl backdrop-blur-xl"
        on:click|stopPropagation
        role="dialog"
        aria-modal="true"
      >
        <!-- Close Button -->
        <button
          class="absolute right-4 top-4 text-gray-400 transition-colors hover:text-white"
          on:click={closeModal}
          aria-label="Close modal"
        >
          <X class="h-6 w-6" />
        </button>

        <!-- Modal Header -->
        <div class="mb-6 text-center">
          {#if modalType === 'file'}
            <FileLock2 class="mx-auto h-16 w-16 text-primary" />
          {:else}
            <FolderLock class="mx-auto h-16 w-16 text-primary" />
          {/if}
          <h3 class="mt-4 text-3xl font-bold">
            Select {modalType === 'file' ? 'File' : 'Folder'}
          </h3>
          <p class="mt-2 text-gray-400">Choose a {modalType} to encrypt</p>
        </div>

        <!-- File Path Display -->
        <div class="mb-6">
          <label class="mb-2 block text-sm font-medium text-gray-300">
            {modalType === 'file' ? 'File' : 'Folder'} Path
          </label>
          <div class="flex items-center gap-3">
            <input
              type="text"
              value={selectedFilePath}
              placeholder="No {modalType} selected"
              readonly
              class="flex-1 rounded-lg border border-white/20 bg-white/5 px-4 py-3 text-white placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-primary/50"
            />
            <button
              on:click={browseFile}
              class="rounded-lg border border-white/20 bg-white/10 px-6 py-3 font-semibold transition-all duration-300 hover:scale-105 hover:border-primary/40 hover:bg-white/20"
            >
              Browse
            </button>
          </div>
        </div>

        <!-- Encrypt Button -->
        <button
          on:click={handleEncrypt}
          disabled={!selectedFilePath}
          class="w-full rounded-lg bg-gradient-to-r from-blue-500 to-cyan-400 px-6 py-4 text-lg font-bold text-white shadow-lg transition-all duration-300 hover:scale-105 hover:shadow-xl disabled:cursor-not-allowed disabled:opacity-40 disabled:hover:scale-100"
        >
          Encrypt {modalType === 'file' ? 'File' : 'Folder'}
        </button>
      </div>
    </div>
  {/if}
</div>

<style>
  :root {
    --x: 50%;
    --y: 50%;
  }
</style>
