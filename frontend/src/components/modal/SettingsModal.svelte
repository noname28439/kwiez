<script>
    import {authToken} from "../../authentication";
    import {onMount} from "svelte";
    import {useEndpoint} from "../../endpoints.js";
    import {stats} from "../stores.js";
    import {sync} from "../networking.js";

    let information;

    stats.subscribe((value) => {
        information = value;
    });

    onMount(() => {
        const input = document.getElementById("stgNameDivTXT");
        input.addEventListener("focus", () => {
        });
    });

    async function renamePlayer() {
        const newName = document.getElementById("stgNameDivTXT").value;
        const res = await useEndpoint("rename", {nickname: newName});
        await sync();
        document.getElementById("stgNameDivTXT").value = '';
        //location.reload()
    }
    async function resetPlayer() {
        const res = await useEndpoint("reset_account");
        location.reload();
    }
</script>

<main>
    <div class="entry">
        <h3 class="settingsHeader">Deine ID</h3>
        <p style="word-wrap: anywhere;">{authToken}</p>
    </div>
    <div class="entry">
        <h3 class="settingsHeader">Dein Benutzername</h3>

        <div id="stgNameDiv">
            <input  maxlength="20"
                    id="stgNameDivTXT"
                    type="text"
                    placeholder={information.progress >= 1
                    ? information.nickname
                        ? information.nickname
                        : "Hier eingeben..."
                    : "Verfügbar ab Frage 2"}
                    disabled={information.progress < 1}
            />

            <button
                    autofocus
                    id="stgNameDivBTN"
                    type="button"
                    on:click={renamePlayer}
                    disabled={information.progress < 1}
            >
                Ändern
            </button>
        </div>
    </div>

    <div class="entry">
        <h3 class="settingsHeader">Zurücksetzen</h3>
        <button
                id="resetBTN"
                type="button"
                on:click={resetPlayer}
        >Account löschen</button>

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

    button:hover {
        cursor: pointer;
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


    #resetBTN {
        border-radius: 0.5em;
        font-weight: 800;
        font-family: "Manrope", sans-serif;
        color: white;
        background-color: rgba(255, 60, 60, 1);
        box-shadow: 0px 4px 0px rgba(0, 0, 0, 0.25);
        border-color: rgb(255, 255, 255, 0.25);
        border-width: 0.15em;
    }
</style>
