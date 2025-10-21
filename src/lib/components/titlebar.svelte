<script lang="ts">
  import { Toolbar } from "bits-ui";
  import { X } from "lucide-svelte";
   import { getCurrentWindow } from '@tauri-apps/api/window';

  function appclose(){
    console.log("Close Initiated");
    getCurrentWindow().close();
  }
</script>

<Toolbar.Root
  class="flex h-14 w-full items-center border-b bg-background px-4"
>
  <div class="flex-1">
    </div>

  <div class="flex-shrink-0">
    <span class="text-lg font-semibold text-foreground">Lockr</span>
  </div>

  <div class="flex flex-1 justify-end">
    <Toolbar.Button
      class="inline-flex h-9 w-9 items-center justify-center rounded-md p-2 text-muted-foreground hover:bg-accent hover:text-accent-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
      aria-label="Close"
      on:click={() => appclose()}
    >
      <X class="h-5 w-5" />
    </Toolbar.Button>
  </div>
</Toolbar.Root>