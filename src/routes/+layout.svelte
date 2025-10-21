<script lang="ts">
    import "../app.css";
    let { children } = $props();
    import Titlebar from "$lib/components/titlebar.svelte";
</script>

<Titlebar />

{@render children()}