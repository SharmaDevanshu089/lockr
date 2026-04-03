<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';
    import { goto } from '$app/navigation';
    import { Breadcrumb, BreadcrumbItem } from "flowbite-svelte";
    import { Heading } from "flowbite-svelte";
    import { GradientButton } from "flowbite-svelte";
    import {gsap} from "gsap";
    function handleFileClick() {
        console.log('File clicked!');
        goto('/decrypt/file');
    }
    function handleFolderClick() {
        console.log('Folder clicked');
        goto('/decrypt/folder');
    }
</script>

<div class="relative w-full overflow-hidden text-primary bg-background">
    <!-- card grid -->
    <div class="relative z-10 min-h-[calc(100vh-8rem)] p-4">
        <Breadcrumb aria-label="Solid background breadcrumb example" >
            <BreadcrumbItem href="/" home>Home</BreadcrumbItem>
            <BreadcrumbItem>Decrypt</BreadcrumbItem>
        </Breadcrumb>
        <Heading tag="h2" class="center mb-4 pt-6">Select between <span class="blue"> files</span> or <span class="blue"> folders. </span> </Heading>
        <div class="herobox">
            <button class="winui-button" on:click={handleFileClick}>
                File
            </button>

            <button class="winui-button" on:click={handleFolderClick}>
                Folder
            </button>

        </div>
    </div>


</div>