<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';
    import { Breadcrumb, BreadcrumbItem } from "flowbite-svelte";
    import { Heading } from "flowbite-svelte";
    import { Spinner } from "flowbite-svelte";
    import {gsap} from "gsap";
    import {onMount} from "svelte";

    const DEBUG = true;

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

    if (DEBUG) {
        console.log("variable filePath:", filePath);
        console.log("variable checked:", checked);
        console.log("variable initialMenu:", initialMenu);
        console.log("variable modalSelect:", modalSelect);
        console.log("variable disabledModalConfirm:", disabledModalConfirm);
        console.log("variable loading:", loading);
        console.log("variable loaderState:", loaderState);
        console.log("variable sucessState:", sucessState);
    }

    onMount(() =>
    {
        heroButton = document.getElementById("HeroButton");
        SpinnerInBrowseButton = document.getElementById("SpinnerInBrowse");
        browseButton = document.getElementById("BrowseText");
        if (DEBUG) {
            console.log("variable heroButton:", heroButton);
            console.log("variable SpinnerInBrowseButton:", SpinnerInBrowseButton);
            console.log("variable browseButton:", browseButton);
        }
    });
    async function openFileDialog(){
        if (DEBUG) {
            console.log("openFileDialog is loading");
        }
        console.log("Browse Clicked ,Starting visual cues");
        SpinnerInBrowseButton.style.display="unset";
        browseButton.style.display="none";
        try {
            console.log("Opening Invoke");
            if (DEBUG) console.log("invoke open_file_dialog");
            fileData = await invoke("open_file_dialog");
            if (DEBUG) console.log("variable fileData:", fileData);
            filename = fileData.filename;
            if (DEBUG) console.log("variable filename:", filename);
            filePath = fileData.filepath;
            if (DEBUG) console.log("variable filePath:", filePath);
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
    async function initiateEncryption(){
        if (DEBUG) {
            console.log("initiateEncryption is loading");
        }
        console.log("Initiating Encryption");
        initialMenu = false;
        modalSelect = true;
        if (DEBUG) {
            console.log("variable initialMenu (updated):", initialMenu);
            console.log("variable modalSelect (updated):", modalSelect);
        }
        if (DEBUG) console.log("invoke generate_key");
        passwordArray = await invoke("generate_key");
        if (DEBUG) console.log("variable passwordArray:", passwordArray);
        console.log(passwordArray);
        password = passwordArray.toString();
        if (DEBUG) console.log("variable password:", password);
        console.log(password);
        disabledModalConfirm = false;
        if (DEBUG) console.log("variable disabledModalConfirm (updated):", disabledModalConfirm);
    }
    async function confirmEncryptionModal(){
        if (DEBUG) {
            console.log("confirmEncryptionModal is loading");
        }
        console.log("Confirming Encryption Directory");
        modalSelect = false;
        loading = true;
        if (DEBUG) {
            console.log("variable modalSelect (updated):", modalSelect);
            console.log("variable loading (updated):", loading);
        }
        if (DEBUG) console.log("invoke get_resulting_dir");
        desktopDirectory = await invoke("get_resulting_dir");
        if (DEBUG) console.log("variable desktopDirectory:", desktopDirectory);
        let encryptionPackageJS = {
            resultant_dir: desktopDirectory,
            password: passwordArray,
            resultname : filename,
            filepath : filePath,
            checked  : checked,
        }
        if (DEBUG) console.log("variable encryptionPackageJS:", encryptionPackageJS);
        loaderState = "Encrypting";
        if (DEBUG) console.log("variable loaderState (updated):", loaderState);
        try {
            if (DEBUG) console.log("invoke final_encryption");
            await invoke("final_encryption", {responsepackage:encryptionPackageJS});
            console.log("Finished Encryption");
            sucessState = true;
            if (DEBUG) console.log("variable sucessState (updated):", sucessState);
        }
        catch (error) {
            console.log(error);
            //TODO: Create a Proper handle for stopping the encryption and popup
        }
        loading = false;
    }
    async function showFileLocation() {
        if (DEBUG) {
            console.log("showFileLocation is loading");
        }
        let newFilename = filename.replace(/\.[^.]+$/, ".aes");
        if (DEBUG) console.log("variable newFilename:", newFilename);
        let new_file_path = desktopDirectory + "\\" + newFilename;
        if (DEBUG) console.log("variable new_file_path:", new_file_path);

        console.log("Path being sent to Rust:", new_file_path);
        try {
            await invoke("open_in_explorer" , { newFilePath: new_file_path });
        }
        catch (error) {
            console.log(error);
        }
    }
</script>
<div class="relative w-full overflow-hidden text-primary bg-background">
    <div class="relative z-10 min-h-[calc(100vh-8rem)] p-4">
        {#if initialMenu}
        <Breadcrumb aria-label="Solid background breadcrumb example" >
            <BreadcrumbItem href="/" home>Home</BreadcrumbItem>
            <BreadcrumbItem href="/encrypt">Encrypt</BreadcrumbItem>
            <BreadcrumbItem>File</BreadcrumbItem>
        </Breadcrumb>
        <Heading tag="h2" class="center mb-4 pt-6">Select the <span class="blue"> file.</span> </Heading>
        <div >
            <div class="winui-filebox">
                <input type="text" value={filePath} disabled />
                <button on:click={openFileDialog}><Spinner type="dots" id="SpinnerInBrowse" size="5" style="display: none" /><text id="BrowseText" style="display:unset">Browse</text></button>
            </div>
            <label class="winui-checkbox">
                <input type="checkbox" bind:checked={checked} />
                <span class="label-text">Also Delete the <b> unencrypted </b>( original ) file</span>
            </label>
        </div>
        <button id="HeroButton" class="winui-button" on:click={initiateEncryption} disabled>
            Start Encryption
        </button>
        {/if}
        {#if modalSelect}
            <div class="dialog-overlay">
                <div class="dialog">
                    <h2 class="dialog-title">Encryption Key</h2>
                    <p class="dialog-warning">
                        ⚠ If the password key is lost, your data will be unrecoverable.<br> There is a Random Key Generated, its recommended you use that and store it somewhere securely!
                    </p>

                    <input
                            type="text"
                            bind:value={password}
                            class="dialog-input"
                            disabled=false
                    />
                    <div class="dialog-actions">
                        <button class="dialog-button" on:click={confirmEncryptionModal} disabled={disabledModalConfirm}>OK</button>
                    </div>
                </div>
            </div>
        {/if}
        {#if loading}
            <Spinner type="orbit" color="rose" />
            <h1>{loaderState}</h1>
        {/if}
        {#if sucessState}
            <h1>Successfully encrypted</h1>
            <button class="winui-button" on:click={showFileLocation}>Show File Location</button>
            <a class="winui-button" href="/">Show File Location</a>
        {/if}
    </div>


</div>