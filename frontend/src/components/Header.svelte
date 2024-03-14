<script>
    import "./ColorPalette.svelte";
</script>

<main>
    <div id="header">
        <div class="iconBox">
            <ion-icon name="stats-chart"></ion-icon>
        </div>
        <div class="iconBox">
            <ion-icon name="settings"></ion-icon>
        </div>
    </div>

    <h1>Schnitzeljagd</h1>

    <div id="aegEvent">
        <img
            src={"/images/einstein.png"}
            alt="Einstein Logo"
            id="einsteinLogo"
        />
        <p style="color: #dbdbdb;">AEG Event</p>
    </div>
</main>

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
</style>
