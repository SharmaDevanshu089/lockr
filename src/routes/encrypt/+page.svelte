<!-- just comment to check workflow -->
<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { goto } from '$app/navigation';
  import { Breadcrumb, BreadcrumbItem } from "flowbite-svelte";
  import { Heading } from "flowbite-svelte";
  import { GradientButton } from "flowbite-svelte";
  import {gsap} from "gsap";


  // let tl = gsap.timeline();
  let hovered: 'encrypt' | 'decrypt' | null = null;
  let showModal = false;
  let selectedFilePath = '';
  let selectedFolderPath = '';
  let modalType: 'file' | 'folder' | null = null;

  function initiateFolderSelection() {
    console.log("Folder clicked!");
    modalType = 'folder';
    showModal = true;
  }

  function initiateFileSelection() {
    console.log('File clicked!');
    modalType = 'file';
    showModal = true;
    // tl.from("#modal",{ x: +500});
  }

  async function browseFile() {
    try {
      // const selected = await open({
      //   multiple: false,
      //   directory: modalType === 'folder'
      // });
      const selected = await invoke("open_file_dialog");


      if (selected) {
        selectedFilePath = selected as string;
      }
    } catch (error) {
      console.error('Error selecting file:', error);
    }
  }

  async function browseFolder() {
    try {
      const selected = await invoke("open_folder_dialog");


      if (selected) {
        selectedFolderPath = selected as string;
      }
    } catch (error) {
      console.error('Error selecting folder:', error);
    }
  }

  function handleEncrypt() {
    if (!selectedFilePath) return;

    console.log(`Encrypting ${modalType}:`, selectedFilePath);
    invoke('get_file_path', { path: selectedFilePath });

    // Reset modal
    closeModal();
  }

  function closeModal() {
    showModal = false;
    selectedFilePath = '';
    modalType = null;
  }

  function updateSpotlight(e: MouseEvent) {
    const root = document.documentElement;
    root.style.setProperty('--x', `${e.clientX}px`);
    root.style.setProperty('--y', `${e.clientY}px`);
  }
</script>

<div class="relative w-full overflow-hidden text-primary bg-background">
  <!-- card grid -->
  <div class="relative z-10 min-h-[calc(100vh-8rem)] p-4">
    <Breadcrumb aria-label="Solid background breadcrumb example" >
      <BreadcrumbItem href="/" home>Home</BreadcrumbItem>
      <BreadcrumbItem>Encrypt</BreadcrumbItem>
    </Breadcrumb>
    <Heading tag="h2" class="center mb-4 pt-6">Select between <span class="blue"> files</span> or <span class="blue"> folders. </span> </Heading>
    <div class="herobox">
      <button class="winui-button">
        File
      </button>

      <button class="winui-button">
        Folder
      </button>

    </div>
  </div>


</div>

<style>
  :root {
    /*color: ;*/
  }
</style>
