<script lang="ts">
    import "../app.css";

    let { children } = $props();
    import * as NavigationMenu from "$lib/components/ui/navigation-menu";
    import { Button } from "$lib/components/ui/button";
    import { X } from "lucide-svelte"; // Imports the 'X' icon
</script>

<NavigationMenu.Root class="w-full h-14 border-b">
    <NavigationMenu.List class="flex w-full items-center px-4">
        <NavigationMenu.Item class="flex-1"></NavigationMenu.Item>

        <NavigationMenu.Item class="font-bold text-lg font-roboto">
            Lockr
        </NavigationMenu.Item>

        <NavigationMenu.Item class="flex flex-1 justify-end">
            <Button
                variant="ghost"
                size="icon"
                class="hover:bg-destructive hover:text-destructive-foreground"
            >
                <X class="h-5 w-5" />
            </Button>
        </NavigationMenu.Item>
    </NavigationMenu.List>
</NavigationMenu.Root>
{@render children()}

<style>
    @import url("https://fonts.googleapis.com/css2?family=Roboto:wght@700&display=swap");

    .font-roboto {
        font-family: "Roboto", sans-serif;
    }
</style>
