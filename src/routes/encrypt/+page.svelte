<script lang="ts">
  import { Lock, Key } from "lucide-svelte";
  // We don't need the click handlers, the <a> tags handle navigation
</script>

<!-- 
  This grid fills the <main> tag from your layout.
  - h-full (makes it fill the space)
  - gap-6 / p-6 (gives the cards space)
-->
<div class="grid h-full grid-cols-1 gap-6 p-6 md:grid-cols-2">
  <!-- 
    1. WRAP THE CARD IN AN <a> TAG
       This makes it a link to the new page.
  -->
  <a
    href="/encrypt"
    data-sveltekit-noscroll
    class="h-full w-full"
  >
    <!-- 
      2. ADD THE view-transition-name
         This MUST match the name on the encrypt/+page.svelte
    -->
    <button
      class="group flex h-full w-full flex-col items-center justify-center rounded-xl border bg-card p-10 text-card-foreground shadow-lg transition-all duration-300 hover:scale-[1.03] hover:bg-accent hover:shadow-xl focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
      style="view-transition-name: main-card;"
    >
      <Lock
        class="h-24 w-24 text-muted-foreground transition-all duration-300 group-hover:scale-110 group-hover:text-primary"
      />
      <h2 class="mt-6 text-5xl font-extrabold text-foreground">Encrypt</h2>
      <p class="mt-2 text-lg text-muted-foreground">
        Secure your files with a password.
      </p>
    </button>
  </a>

  <!-- 
    3. DO THE SAME FOR THE DECRYPT CARD
       (You'll need to create /decrypt/+page.svelte later)
  -->
  <a
    href="/decrypt"
    data-sveltekit-noscroll
    class="h-full w-full"
  >
    <button
      class="group flex h-full w-full flex-col items-center justify-center rounded-xl border bg-card p-10 text-card-foreground shadow-lg transition-all duration-300 hover:scale-[1.03] hover:bg-accent hover:shadow-xl focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
      style="view-transition-name: decrypt-card;"
    >
      <Key
        class="h-24 w-24 text-muted-foreground transition-all duration-300 group-hover:scale-110 group-hover:text-primary"
      />
      <h2 class="mt-6 text-5xl font-extrabold text-foreground">Decrypt</h2>
      <p class="mt-2 text-lg text-muted-foreground">
        Unlock your secured files.
      </p>
    </button>
  </a>
</div>