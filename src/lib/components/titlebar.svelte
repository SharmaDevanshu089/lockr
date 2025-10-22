<script lang="ts">
  import { Toolbar } from "bits-ui";
  import { X } from "lucide-svelte";
  import { getCurrentWindow } from "@tauri-apps/api/window";

  function appclose() {
    console.log("Close Initiated");
    getCurrentWindow().close();
  }
</script>

<Toolbar.Root
  class="flex h-14 w-full items-center border-b bg-background px-4 cursor-grab active:cursor-grabbing "
  data-tauri-drag-region
>
  <div class="flex-1"></div>

  <div class="shrink-0">
    <span class="text-x font-semibold text-foreground">Lockr</span>
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