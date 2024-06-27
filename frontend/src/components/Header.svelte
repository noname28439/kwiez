<script>
    import "./ColorPalette.svelte";
    import Modal from "./modal/Modal.svelte";
    import RankingModal from "./modal/RankingModal.svelte";
    import SettingsModal from "./modal/SettingsModal.svelte";

    let showModal = false;
    let selectedModal = "rankings";
</script>

<main>
    <div id="header">
        <button
            id="divBtn"
            on:click={() => {
                showModal = true;
                selectedModal = "rankings";
            }}
        >
            <div class="iconBox">
                <ion-icon name="stats-chart"></ion-icon>
            </div>
        </button>

        <button
            id="divBtn"
            on:click={() => {
                showModal = true;
                selectedModal = "settings";
            }}
        >
            <div class="iconBox">
                <ion-icon name="settings"></ion-icon>
            </div>
        </button>
    </div>

    <h1>AEG QUIZ</h1>

    <div id="aegEvent">
        <img
            src={"/images/einstein.png"}
            alt="Einstein Logo"
            id="einsteinLogo"
        />
        <p style="color: #dbdbdb;">Abistreich Event</p>
    </div>
</main>

{#if selectedModal == "settings"}
    <Modal bind:showModal>
        <h2 slot="header">Einstellungen</h2>
        <SettingsModal/>
    </Modal>
{:else if selectedModal == "rankings"}
    <Modal bind:showModal>
        <h2 slot="header">Rankings</h2>
        <RankingModal/>
    </Modal>
{/if}



<style>
    main {
        width: 100%;
    }

    #header {
        width: 100%;
        display: flex;
        flex: 1;
        flex-direction: row;
        justify-content: space-between;

        margin-bottom: 1em;
    }

    .iconBox {
        display: flex;
        border-radius: 20%;
    }

    .iconBox ion-icon {
        font-size: 1.5em;
    }

    #einsteinLogo {
        opacity: 0.1;
        width: 1.5em;
        margin-right: 0.5em;
    }

    #aegEvent {
        display: flex;
        flex-direction: row;
        align-items: center;
    }

    #aegEvent p {
        font-weight: 600;
    }

    #divBtn {
        border-width: 0;
        border-color: var(--background-color);
        border-radius: 25%;
        padding: 0.5em;
        margin: 0;
    }

    #divBtn:hover {
        cursor: pointer;
    }
</style>
