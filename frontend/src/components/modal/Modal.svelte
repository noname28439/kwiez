<script>
    export let showModal; // boolean

    let dialog; // HTMLDialogElement

    $: if (dialog && showModal) dialog.showModal();
</script>

<!-- svelte-ignore a11y-click-events-have-key-events a11y-no-noninteractive-element-interactions -->
<dialog
    bind:this={dialog}
    on:close={() => (showModal = false)}
    on:click|self={() => dialog.close()}
>
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div on:click|stopPropagation>
        <div id="headerDiv">
            <slot name="header" />
            <div class="iconBox" on:click={() => dialog.close()}>
                <ion-icon name="close"></ion-icon>
            </div>
        </div>

        <slot />
    </div>
</dialog>

<style>
    dialog {
        max-width: 32em;
        border-radius: 2em;
        border: none;
        padding: 0;
        box-shadow: 0px 4px 0px rgba(0, 0, 0, 0.25);
    }
    dialog::backdrop {
        background: rgba(0, 0, 0, 0.3);
    }
    dialog > div {
        padding: 2em;
    }
    dialog[open] {
        animation: zoom 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
    }
    @keyframes zoom {
        from {
            transform: scale(0.95);
        }
        to {
            transform: scale(1);
        }
    }
    dialog[open]::backdrop {
        animation: fade 0.2s ease-out;
    }
    @keyframes fade {
        from {
            opacity: 0;
        }
        to {
            opacity: 1;
        }
    }

    /*----------------*/

    #headerDiv {
        display: flex;
        flex-direction: row;
        justify-content: space-between;
        margin-bottom: 1em;
    }
    .iconBox {
        display: flex;
        justify-content: center;
        align-items: center;
    }

    .iconBox:hover {
        cursor: pointer;
    }

    .iconBox ion-icon {
        font-size: 1.5em;
    }
</style>
