//For empty commit
<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';
    import { Breadcrumb, BreadcrumbItem } from "flowbite-svelte";
    import { Heading } from "flowbite-svelte";
    import { Spinner } from "flowbite-svelte";
    import {gsap} from "gsap";
    import {onMount} from "svelte";


    let filePath = "File Not Selected";
    let fileData;
    let checked = false;
    let browseButton: HTMLElement;
    let SpinnerInBrowseButton: HTMLElement;
    let heroButton: HTMLElement;
    let initialMenu = true;
    let modalSelect = false;
    let password ;
    let disabledModalConfirm = true;
    let passwordArray: [u8,32];
    let loading = false;
    let loaderState = "Initializing...";
    let desktopDirectory;
    let filename;
    let sucessState = false;

    onMount(() =>
    {
        heroButton = document.getElementById("HeroButton");
        SpinnerInBrowseButton = document.getElementById("SpinnerInBrowse");
        browseButton = document.getElementById("BrowseText");
    });

    async function initiateDecryption() {
        initialMenu = false;
        modalSelect = true;
        disabledModalConfirm = false;
    }

    async function openFileDialog(){
        console.log("Browse Clicked ,Starting visual cues");
        SpinnerInBrowseButton.style.display="unset";
        browseButton.style.display="none";
        try {
            console.log("Opening Invoke");
            fileData = await invoke("open_file_dialog");
            filename = fileData.filename;
            filePath = fileData.filepath;
            console.log("Stopping Visual Cues");
            SpinnerInBrowseButton.style.display="none";
            browseButton.style.display="unset";
            heroButton.disabled = false;
        }
        catch (error) {
            console.log("Error Condition is ran");
            console.log(error);
            SpinnerInBrowseButton.style.display="none";
            browseButton.style.display="unset";
        }
    }
</script>


<div class="relative w-full overflow-hidden text-primary bg-background">
    <div class="relative z-10 min-h-[calc(100vh-8rem)] p-4">
        {#if initialMenu}
            <Breadcrumb aria-label="Solid background breadcrumb example" >
                <BreadcrumbItem href="/" home>Home</BreadcrumbItem>
                <BreadcrumbItem href="/decrypt">Decrypt</BreadcrumbItem>
                <BreadcrumbItem>File</BreadcrumbItem>
            </Breadcrumb>
            <Heading tag="h2" class="center mb-4 pt-6">Select the <span class="blue"> file.</span> </Heading>
            <div >
                <div class="winui-filebox">
                    <input type="text" value={filePath} disabled />
                    <button on:click={openFileDialog}><Spinner type="dots" id="SpinnerInBrowse" size="5" style="display: none" /><text id="BrowseText" style="display:unset">Browse</text></button>
                </div>
            </div>
            <button id="HeroButton" class="winui-button" on:click={initiateDecryption} disabled>
                Start Decryption
            </button>
        {/if}
        {#if modalSelect}
            <div class="dialog-overlay">
                <div class="dialog">
                    <h2 class="dialog-title">Enter the Encryption Key</h2>
                    <p class="dialog-warning">
                        ⚠ Please Enter the key which was entered or generated during the time of encryption.<br> Without this key you cannot Decrypt the file.
                    </p>
                    <input
                            type="text"
                            bind:value={password}
                            class="dialog-input"
                    />
                    <div class="dialog-actions">
                        <button class="dialog-button" on:click={confirmEncryptionModal} disabled={disabledModalConfirm}>OK</button>
                    </div>
                </div>
            </div>
        {/if}
    </div>
</div>