<script lang="ts">
  import { Lock, Key } from "lucide-svelte";
  import { goto } from '$app/navigation';

  let hovered: 'encrypt' | 'decrypt' | null = null;
  let isExpanded = false;
  let isDecryptedExpanded = false;
function handleDecryptClick() {
    const el = document.getElementById("decrypt");
    if (!el || isDecryptedExpanded) return;
    isDecryptedExpanded = true;

    const rect = el.getBoundingClientRect();

    // fix the card's position and size
    el.style.position = "fixed";
    el.style.top = rect.top + "px";
    el.style.left = rect.left + "px";
    el.style.width = rect.width + "px";
    el.style.height = rect.height + "px";
    el.style.zIndex = "1000";
    el.style.transition = "all 0.5s ease";

    // expand to full screen next tick
    requestAnimationFrame(() => {
      el.style.top = "0";
      el.style.left = "0";
      el.style.width = "100vw";
      el.style.height = "100vh";
    });

    console.log("Decrypt clicked!");
      setTimeout(() => {
    goto('/decrypt'); // your new route
  }, 500);
}
  function handleEncryptClick() {
      const el = document.getElementById("encrypt");
  if (!el || isExpanded) return;
  isExpanded = true;

  const rect = el.getBoundingClientRect();

  // fix the card's position and size
  el.style.position = "fixed";
  el.style.top = rect.top + "px";
  el.style.left = rect.left + "px";
  el.style.width = rect.width + "px";
  el.style.height = rect.height + "px";
  el.style.zIndex = "1000";
  el.style.transition = "all 0.5s ease";

  // expand to full screen next tick
  requestAnimationFrame(() => {
    el.style.top = "0";
    el.style.left = "0";
    el.style.width = "100vw";
    el.style.height = "100vh";
  });
    console.log('Encrypt clicked!');
      setTimeout(() => {
    goto('/encrypt'); // your new route
  }, 500);
  }

  // spotlight cursor effect
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
      on:click={handleEncryptClick}
      on:keydown={(e) => (e.key === 'Enter' || e.key === ' ') && handleEncryptClick()}
    >
      <div class="text-center transition-all duration-500" class:opacity-80={hovered === 'decrypt'}>
        <Lock class="mx-auto h-24 w-24 text-primary transition-all duration-500 group-hover:scale-110" />
        <h2 class="mt-6 text-5xl font-extrabold">Encrypt</h2>
        <p class="mt-3 text-lg text-gray-300">Secure your files with a password.</p>
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
        <Key class="mx-auto h-24 w-24 text-primary transition-all duration-500 group-hover:scale-110" />
        <h2 class="mt-6 text-5xl font-extrabold">Decrypt</h2>
        <p class="mt-3 text-lg text-gray-300">Unlock your secured files.</p>
      </div>

      <div
        class="absolute -inset-0.5 rounded-3xl bg-linear-to-r from-cyan-400/20 to-blue-500/20 opacity-0 blur-xl transition-opacity duration-500 group-hover:opacity-100 pointer-events-none"
      ></div>
    </div>
  </div>
</div>

<style>
  :root {
    --x: 50%;
    --y: 50%;
  }
</style>
