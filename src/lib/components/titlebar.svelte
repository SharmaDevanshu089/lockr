<script lang="ts">
  import { Toolbar } from "bits-ui";
  import { X } from "lucide-svelte";
  import { getCurrentWindow } from "@tauri-apps/api/window";

  function appclose() {
    console.log("Close Initiated");
    getCurrentWindow().close();
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
id = "navbar"
  class="flex h-18 w-full items-center px-4 cursor-grab active:cursor-grabbing transition duration-300 shadow-xs"
  data-tauri-drag-region
  onmouseover={() => addBorder()}
  onmouseleave={() => removerBorder()}
>
  <div class="flex-1"></div>

  <div class="shrink-0">
    <span class="text-4xl font-semibold text-foreground">Lockr</span>
  </div>

  <div class="flex flex-1 justify-end">
    <button
      class="inline-flex h-9 w-9 items-center justify-center rounded-md p-2 text-muted-foreground hover:bg-accent hover:text-accent-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
      aria-label="Close"
      onclick={() => appclose()}
    >
      <X class="h-5 w-5" />
    </button>
  </div>
</Toolbar.Root>