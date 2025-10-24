<svelte:head>
  <link rel="prefetch" href="/encrypt" />
  <link rel="prefetch" href="/decrypt" />
</svelte:head>
<script lang="ts">
  import "../app.css";
  import Titlebar from "$lib/components/titlebar.svelte";
  import { page } from "$app/stores";
  import { slide, fade } from "svelte/transition";
  import { openUrl } from '@tauri-apps/plugin-opener';
  import { ContextMenu } from "bits-ui";
  import { Bug, Folder, User } from "lucide-svelte"; // lucide icons

  let { children } = $props();

  async function openGithub() {
    await openUrl("https://github.com/SharmaDevanshu089");
  }

  function addFocus() {
    const el = document.getElementById("devanshu-span");
    el?.classList.remove('text-muted-foreground');
    el?.classList.add('shadow-xl', 'text-primary'); 
  }

  function removeFocus() {
    const el = document.getElementById("devanshu-span");
    el?.classList.remove('shadow-xl', 'text-primary');
    el?.classList.add('text-muted-foreground');
  }

  function handleIssues() { openUrl("https://github.com/SharmaDevanshu089/lockr"); }
  function handleSource() { openUrl("https://github.com/SharmaDevanshu089/lockr/issues");}
  function handleAbout() { openUrl("https://github.com/SharmaDevanshu089");}
</script>

<Titlebar />

<!-- wrap the whole app content in ContextMenu -->
<ContextMenu.Root>
  <ContextMenu.Trigger>
    <div class="flex h-[calc(100vh-4.5rem)] flex-col bg-background">

      <main class="flex-1 overflow-y-auto">
        {#key $page.url.pathname}
          <div in:fade={{ duration: 150 }} out:fade={{ duration: 150 }}>
            {@render children()}
          </div>
        {/key}
      </main>

      <footer class="shrink-0 p-4 text-center">
        <span
          id="devanshu-span"
          class="text-sm text-muted-foreground span rounded-md transition-all duration-300 focus:outline-none focus:ring-2 focus:ring-ring"
          role="link"
          tabindex="0"
          onmouseover={addFocus}
          onfocus={addFocus}
          onmouseout={removeFocus}
          onblur={removeFocus}
          onclick={openGithub}
          onkeydown={(e) => e.key === 'Enter' && openGithub()} 
        >
          Made by Devanshu
        </span>
      </footer>

    </div>
  </ContextMenu.Trigger>

<ContextMenu.Portal>
  <ContextMenu.Content class="border-muted bg-background shadow-popover w-[220px] rounded-xl border px-1 py-1.5 outline-none focus-visible:outline-none z-9999">

    <ContextMenu.Item class="rounded-button data-highlighted:bg-muted flex h-10 select-none items-center py-3 pl-3 pr-1.5 text-sm font-medium focus-visible:outline-none">
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <div onclick={() => handleIssues()} class="flex items-center gap-3 cursor-pointer">
        <Bug class="h-5 w-5" />Issues
      </div>
    </ContextMenu.Item>

    <ContextMenu.Item class="rounded-button data-highlighted:bg-muted flex h-10 select-none items-center py-3 pl-3 pr-1.5 text-sm font-medium focus-visible:outline-none">
      <div onclick={() => handleSource()} class="flex items-center gap-3 cursor-pointer">
        <Folder class="h-5 w-5" />Source
      </div>
    </ContextMenu.Item>

    <ContextMenu.Separator class="bg-muted -mx-1 my-1 block h-px" />

    <ContextMenu.Item class="rounded-button data-highlighted:bg-muted flex h-10 select-none items-center py-3 pl-3 pr-1.5 text-sm font-medium focus-visible:outline-none">
      <div onclick={() => handleAbout()} class="flex items-center gap-3 cursor-pointer">
        <User class="h-5 w-5" />About Me
      </div>
    </ContextMenu.Item>

  </ContextMenu.Content>
</ContextMenu.Portal>

</ContextMenu.Root>
