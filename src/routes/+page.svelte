<script lang="ts">
  import { Lock, Key } from "lucide-svelte";

  function handleEncryptClick() {
    console.log("Encrypt clicked!");
    // Navigate to your encrypt page or show a modal
  }

  function handleDecryptClick() {
    console.log("Decrypt clicked!");
    // Navigate to your decrypt page or show a modal
  }
</script>

<div class="flex h-[calc(100vh-4.5rem)] flex-col bg-background">
  <main class="flex-1 grid grid-cols-1 gap-6 p-6 md:grid-cols-2">
    <button
      class="group flex h-full w-full flex-col items-center justify-center rounded-xl border bg-card p-10 text-card-foreground shadow-lg transition-all duration-300 hover:scale-[1.03] hover:bg-accent hover:shadow-xl focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
      on:click={handleEncryptClick}
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
      on:click={handleDecryptClick}
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

  <footer class="flex-shrink-0 p-4 text-center">
    <span class="text-sm text-muted-foreground">Made by Devanshu</span>
  </footer>
</div>