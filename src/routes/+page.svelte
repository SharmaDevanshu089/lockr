<script lang="ts">
  import { Lock, Key } from "lucide-svelte";

  let hovered: 'encrypt' | 'decrypt' | null = null;

  function handleEncryptClick() {
    console.log('Encrypt clicked!');
  }

  function handleDecryptClick() {
    console.log('Decrypt clicked!');
  }

  // spotlight cursor effect
  function updateSpotlight(e: MouseEvent) {
    const root = document.documentElement;
    root.style.setProperty('--x', `${e.clientX}px`);
    root.style.setProperty('--y', `${e.clientY}px`);
  }
</script>

<!-- Background with reactive spotlight -->
<div
  class="relative min-h-screen w-full overflow-hidden bg-gradient-to-br from-[#0a0a0a] via-[#111] to-[#191919] text-white"
  on:mousemove={updateSpotlight}
>
  <div
    class="absolute inset-0 transition-all duration-300"
    style="background: radial-gradient(400px circle at var(--x, 50%) var(--y, 50%), rgba(100, 200, 255, 0.12), transparent 40%);"
  ></div>

  <div class="relative z-10 grid h-full grid-cols-1 gap-8 p-10 md:grid-cols-2">
    <!-- Encrypt Panel -->
    <div
      role="button"
      tabindex="0"
      class="group relative flex h-[60vh] cursor-pointer items-center justify-center rounded-3xl border border-white/20 bg-white/10 p-10 shadow-2xl backdrop-blur-xl transition-all duration-500 hover:scale-[1.03] hover:border-primary/40"
      on:mouseenter={() => (hovered = 'encrypt')}
      on:mouseleave={() => (hovered = null)}
      on:click={handleEncryptClick}
      on:keydown={(e) => e.key === 'Enter' && handleEncryptClick()}
    >
      <div class="text-center transition-all duration-500" class:opacity-80={hovered === 'decrypt'}>
        <Lock class="mx-auto h-24 w-24 text-primary transition-all duration-500 group-hover:scale-110" />
        <h2 class="mt-6 text-5xl font-extrabold">Encrypt</h2>
        <p class="mt-3 text-lg text-gray-300">Secure your files with a password.</p>
      </div>

      <!-- subtle glow -->
      <div
        class="absolute -inset-0.5 rounded-3xl bg-gradient-to-r from-blue-500/20 to-cyan-400/20 opacity-0 blur-xl transition-opacity duration-500 group-hover:opacity-100 pointer-events-none"
      ></div>
    </div>

    <!-- Decrypt Panel -->
    <div
      role="button"
      tabindex="0"
      class="group relative flex h-[60vh] cursor-pointer items-center justify-center rounded-3xl border border-white/20 bg-white/10 p-10 shadow-2xl backdrop-blur-xl transition-all duration-500 hover:scale-[1.03] hover:border-primary/40"
      on:mouseenter={() => (hovered = 'decrypt')}
      on:mouseleave={() => (hovered = null)}
      on:click={handleDecryptClick}
      on:keydown={(e) => e.key === 'Enter' && handleDecryptClick()}
    >
      <div class="text-center transition-all duration-500" class:opacity-80={hovered === 'encrypt'}>
        <Key class="mx-auto h-24 w-24 text-primary transition-all duration-500 group-hover:scale-110" />
        <h2 class="mt-6 text-5xl font-extrabold">Decrypt</h2>
        <p class="mt-3 text-lg text-gray-300">Unlock your secured files.</p>
      </div>

      <div
        class="absolute -inset-0.5 rounded-3xl bg-gradient-to-r from-cyan-400/20 to-blue-500/20 opacity-0 blur-xl transition-opacity duration-500 group-hover:opacity-100 pointer-events-none"
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