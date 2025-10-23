<script lang="ts">
  import { Toolbar } from "bits-ui";
  import { X, ArrowLeft } from "lucide-svelte";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';

  function appclose() {
    console.log("Close Initiated");
    getCurrentWindow().close();
  }

  function goBack() {
    goto('/');
  }

  function addBorder() {
    document.getElementById("navbar")?.classList.add('shadow-xl');
    document.getElementById("navbar")?.classList.remove('shadow-xs');
  }
  function removerBorder() {
    document.getElementById("navbar")?.classList.remove('shadow-xl')
  }
</script>

<Toolbar.Root
  id="navbar"
  class="relative flex h-18 w-full items-center px-4 cursor-grab active:cursor-grabbing transition duration-300 shadow-xs"
  data-tauri-drag-region
  onmouseover={addBorder}
  onmouseleave={removerBorder}
>
  <!-- Left side: back button -->
  <div class="flex items-center z-20">
    {#if $page.url.pathname !== '/'}
      <button
        class="shrink-0 mr-4 flex h-9 w-9 items-center justify-center rounded-xl p-2 text-muted-foreground hover:bg-accent hover:text-accent-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
        aria-label="Back"
        on:click={goBack}
        data-tauri-drag-region
      >
        <ArrowLeft class="h-5 w-5" />
      </button>
    {/if}
  </div>

  <!-- Centered logo -->
  <div class="absolute left-1/2 transform -translate-x-1/2 z-10" data-tauri-drag-region>
    <span class="text-4xl font-semibold text-foreground">Lockr</span>
  </div>

  <!-- Right side: close button -->
  <div class="flex flex-1 justify-end z-20" data-tauri-drag-region>
    <button
      class="close inline-flex h-9 w-9 items-center justify-center rounded-xl p-2 text-muted-foreground hover:bg-accent hover:text-accent-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring hover:border-accent"
      aria-label="Close"
      on:click={appclose}
      data-tauri-drag-region
    >
      <X class="h-5 w-5" />
    </button>
  </div>
</Toolbar.Root>
