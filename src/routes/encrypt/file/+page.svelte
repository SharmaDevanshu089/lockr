<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';
    import { goto } from '$app/navigation';
    import { Breadcrumb, BreadcrumbItem } from "flowbite-svelte";
    import { Heading } from "flowbite-svelte";
    import { GradientButton } from "flowbite-svelte";
    import {gsap} from "gsap";
</script>

<div class="relative w-full overflow-hidden text-primary bg-background">
    <div class="relative z-10 min-h-[calc(100vh-8rem)] p-4">
        <Breadcrumb aria-label="Solid background breadcrumb example" >
            <BreadcrumbItem href="/" home>Home</BreadcrumbItem>
            <BreadcrumbItem href="/encrypt">Encrypt</BreadcrumbItem>
            <BreadcrumbItem>File</BreadcrumbItem>
        </Breadcrumb>
        <Heading tag="h2" class="center mb-4 pt-6">Select the <span class="blue"> file.</span> </Heading>
        <div class="herobox">
            <button class="winui-button">
                Select
            </button>
            

        </div>
    </div>


</div>