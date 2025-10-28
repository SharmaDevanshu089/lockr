<script lang="ts">
  import { Dialog, Label, Separator } from "bits-ui";
  import { LockKeyhole, X } from "lucide-svelte";

  export let open = false;         // you control this externally
  export let onClose: () => void;  // callback when closed
</script>

{#if open}
  <Dialog.Root open onOpenChange={(v) => !v && onClose?.()}>
    <Dialog.Portal>
      <Dialog.Overlay
        class="data-[state=open]:animate-in data-[state=closed]:animate-out
               data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0
               fixed inset-0 z-40 bg-black/70 backdrop-blur-md"
      />

      <Dialog.Content
        class="rounded-xl bg-background/80 backdrop-blur-xl shadow-2xl
               data-[state=open]:animate-in data-[state=closed]:animate-out
               data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0
               data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95
               outline-hidden fixed left-1/2 top-1/2 z-50 w-full
               max-w-[460px] -translate-x-1/2 -translate-y-1/2 border border-white/10 p-6"
      >
        <Dialog.Title class="text-lg font-semibold tracking-tight text-center">
          Encrypt File
        </Dialog.Title>

        <Separator.Root class="bg-muted -mx-5 mb-6 mt-5 block h-px" />

        <div class="flex flex-col gap-1 pb-10 pt-6">
          <Label.Root for="file" class="text-sm font-medium text-muted-foreground">
            Select File
          </Label.Root>

          <div class="relative w-full">
            <input
              id="file"
              class="h-input rounded-md border border-border-input
                     bg-background placeholder:text-foreground-alt/50
                     hover:border-dark/40 focus:ring-foreground
                     focus:ring-offset-background focus:outline-hidden
                     inline-flex w-full items-center px-4 text-base focus:ring-2 focus:ring-offset-2 sm:text-sm"
              placeholder="Choose file path"
            />
            <LockKeyhole class="text-dark/30 absolute right-4 top-[14px] size-[20px]" />
          </div>
        </div>

        <div class="flex w-full justify-end gap-3">
          <button
            class="rounded-input bg-muted text-muted-foreground hover:bg-muted/80 px-6 py-2 text-sm font-semibold transition-all active:scale-[0.97]"
            on:click={() => onClose?.()}
          >
            Cancel
          </button>

          <button
            class="rounded-input bg-dark text-background shadow-mini hover:bg-dark/90 px-6 py-2 text-sm font-semibold transition-all active:scale-[0.97]"
            on:click={() => onClose?.()}
          >
            Encrypt
          </button>
        </div>

        <button
          class="focus-visible:ring-foreground focus-visible:ring-offset-background
                 absolute right-5 top-5 rounded-md focus-visible:ring-2 focus-visible:ring-offset-2 active:scale-[0.98]"
          on:click={() => onClose?.()}
        >
          <X class="text-foreground size-5" />
        </button>
      </Dialog.Content>
    </Dialog.Portal>
  </Dialog.Root>
{/if}
