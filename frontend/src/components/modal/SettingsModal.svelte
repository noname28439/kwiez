<script>
    import {authToken} from "../../authentication";
    import {onMount} from "svelte";
    import {useEndpoint} from "../../endpoints.js";

    export let playerName;
    export let stats;


    onMount(() => {
        const input = document.getElementById("stgNameDivTXT");
        input.addEventListener("focus", () => {
            console.log("Input focused");
        });
    });



    async function renamePlayer() {
        const newName = document.getElementById('stgNameDivTXT').value
        const res = await useEndpoint("rename", {nickname:newName})

        location.reload()
    }



</script>

<main>
    <div class="entry">
        <h3 class="settingsHeader">Deine ID:</h3>
        <p style="word-wrap: anywhere;">{authToken}</p>
    </div>
    <div class="entry">
        <h3 class="settingsHeader">Dein Benutzername:</h3>

        <div id="stgNameDiv">
            {#if stats.progress >= 1}
                <input
                        id="stgNameDivTXT"
                        type="text"
                        placeholder={playerName}
                />

                <button autofocus id="stgNameDivBTN" type="button" on:click={renamePlayer}>Ändern</button>
            {:else}

                <input
                        id="stgNameDivTXT"
                        type="text"
                        placeholder={"Verfügbar ab Frage 2"}
                        disabled
                />

                <button autofocus id="stgNameDivBTN" type="button" on:click={renamePlayer} disabled>Ändern</button>

            {/if}


        </div>
    </div>
</main>

<style>
    p {
        font-weight: 500;
    }

    .entry {
        margin-bottom: 2em;
    }

    input {
        font-family: "Manrope";
    }

    #stgNameDiv {
        width: 100%;
        display: flex;
        flex-direction: row;
        align-items: center;
    }

    #stgNameDivTXT {
        width: 100%;
        font-weight: 800;
        font-family: "Manrope", sans-serif;
        width: 100%;
        border-width: 0.15em;
        border-radius: 0.5em;

        box-shadow: 0px 4px 0px rgba(0, 0, 0, 0.25);
    }

    #stgNameDivBTN {
        border-radius: 0.5em;
        font-weight: 800;
        font-family: "Manrope", sans-serif;
        color: white;
        background-color: rgba(0, 0, 0, 0.54);
        border-width: 0;
        box-shadow: 0px 4px 0px rgba(0, 0, 0, 0.25);
        margin-left: 0.5em;
        border-width: 0.15em;
    }

    .settingsHeader {
        margin-top: 1em;
        margin-bottom: 0.5em;
    }
</style>
