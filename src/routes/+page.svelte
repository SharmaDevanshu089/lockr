<script lang="ts">
  import { Lock, Key } from "lucide-svelte";
  import { goto } from '$app/navigation';
  import { Breadcrumb, BreadcrumbItem } from "flowbite-svelte";
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

<div class="relative w-full overflow-hidden text-white">

  <!-- card grid -->
<!--  <div class="relative z-10 grid min-h-[calc(100vh-8rem)] grid-cols-1 gap-8 p-10 md:grid-cols-2">-->
    <Breadcrumb aria-label="Solid background breadcrumb example">
      <BreadcrumbItem href="/" home>Home</BreadcrumbItem>
      <BreadcrumbItem href="/">Projects</BreadcrumbItem>
      <BreadcrumbItem>Flowbite Svelte</BreadcrumbItem>
    </Breadcrumb>
<!--  </div>-->

</div>

<style>
  :root {
    --x: 50%;
    --y: 50%;
  }
</style>
