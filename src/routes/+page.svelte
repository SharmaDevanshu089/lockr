<script lang="ts">
  import { Lock, Key } from "lucide-svelte";
import { openUrl } from '@tauri-apps/plugin-opener';
  function handleEncryptClick() {
    console.log("Encrypt clicked!");
    // Navigate to your encrypt page or show a modal
  }

  function handleDecryptClick() {
    console.log("Decrypt clicked!");
    // Navigate to your decrypt page or show a modal
  }
    async function openGithub() {
    await openUrl("https://github.com/SharmaDevanshu089/ac-installer-gui");
    // await console.log("The Button is Pressed");
  }
function addFocus() {
    const el = document.getElementById("devanshu-span");
    el?.classList.remove('text-muted-foreground');
    // Add the "focused" classes
    el?.classList.add('shadow-xl', 'text-primary'); 
  }

  function removeFocus() {
    const el = document.getElementById("devanshu-span");
    // Remove the "focused" classes
    el?.classList.remove('shadow-xl', 'text-primary');
    el?.classList.add('text-muted-foreground');
  }
</script>

<div class="flex h-[calc(100vh-4.5rem)] flex-col bg-background">
  <main class="flex-1 grid grid-cols-1 gap-6 p-6 md:grid-cols-2">
    <button
      class="group flex h-full w-full flex-col items-center justify-center rounded-xl border bg-card p-10 text-card-foreground shadow-lg transition-all duration-300 hover:scale-[1.03] hover:bg-accent hover:shadow-xl focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
    >
      <Lock
        class="h-24 w-24 text-muted-foreground transition-all duration-300 group-hover:scale-110 group-hover:text-primary"
      />
      <h2 class="mt-6 text-5xl font-extrabold text-foreground">Encrypt</h2>
      <p class="mt-2 text-lg text-muted-foreground">
        Secure your files with a password.
      </p>
    </button>

    <button
      class="group flex h-full w-full flex-col items-center justify-center rounded-xl border bg-card p-10 text-card-foreground shadow-lg transition-all duration-300 hover:scale-[1.03] hover:bg-accent hover:shadow-xl focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
    >
      <Key
        class="h-24 w-24 text-muted-foreground transition-all duration-300 group-hover:scale-110 group-hover:text-primary"
      />
      <h2 class="mt-6 text-5xl font-extrabold text-foreground">Decrypt</h2>
      <p class="mt-2 text-lg text-muted-foreground">
        Unlock your secured files.
      </p>
    </button>
  </main>

  <footer class="shrink-0 p-4 text-center">
    <span
    id="devanshu-span" class="text-sm text-muted-foreground span rounded-md transition-all duration-300 focus:outline-none focus:ring-2 focus:ring-ring"
    role="link"
    tabindex="0"
    onmouseover={addFocus}
    onfocus={addFocus}
    onmouseout={removeFocus}
    onblur={removeFocus}
  >
    Made by Devanshu
  </span>
  </footer>
</div>