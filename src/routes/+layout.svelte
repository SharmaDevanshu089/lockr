<script lang="ts">
  import "../app.css";
  import Titlebar from "$lib/components/titlebar.svelte";
  import { fade } from "svelte/transition";
  import { page } from "$app/stores";import { openUrl } from '@tauri-apps/plugin-opener';
  let { children } = $props();

  async function openGithub() {
    // 2. Renamed 'openUrl' to 'open'
    await open("https://github.com/SharmaDevanshu089");
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
</script>

<Titlebar />

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
      onclick={openGithub} onkeydown={(e) => e.key === 'Enter' && openGithub()} >
      Made by Devanshu
    </span>
  </footer>
</div>