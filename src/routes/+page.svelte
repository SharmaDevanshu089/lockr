<script lang="ts">
  import { Lock, Key } from "lucide-svelte";
  import { goto } from '$app/navigation';
  import { Breadcrumb, BreadcrumbItem } from "flowbite-svelte";
  import { Heading } from "flowbite-svelte";
  import { GradientButton } from "flowbite-svelte";
  import {gsap} from "gsap";

  function handleEncryptClick() {
    console.log('Encrypt clicked!');
    goto('/encrypt');
  }
  function handleDecryptClick() {
    console.log('Encrypt clicked!');
    goto('/decrypt');
  }
</script>

<div class="relative w-full overflow-hidden text-primary bg-background">
  <!-- card grid -->
<div class="relative z-10 min-h-[calc(100vh-8rem)] p-4">
    <Breadcrumb aria-label="Solid background breadcrumb example">
      <BreadcrumbItem href="/" home>Home</BreadcrumbItem>
    </Breadcrumb>
  <Heading tag="h2" class="center mb-4 pt-6">Encrypt your <span class="blue"> files and folders.</span></Heading>
    <div class="herobox">
        <button class="winui-button" on:click={handleEncryptClick}>
            Encrypt
        </button>

        <button class="winui-button" on:click={handleDecryptClick}>
            Decrypt
        </button>

    </div>
  </div>


</div>

<style>
  :root {
    /*color: ;*/
  }
</style>
