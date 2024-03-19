<script>
    import { authToken } from "../../authentication";
    import { onMount } from "svelte";

    onMount(() => {
        const input = document.getElementById("stgNameDivTXT");
        input.addEventListener("focus", () => {
            console.log("Input focused");
        });
    });
</script>

<main>
    <div class="entry">
        <h3 class="settingsHeader">Deine ID:</h3>
        <p style="word-wrap: anywhere;">{authToken}</p>
    </div>
    <div class="entry">
        <h3 class="settingsHeader">Dein Benutzername:</h3>

        <div id="stgNameDiv">
            <input
                id="stgNameDivTXT"
                type="text"
                placeholder="Manfredkiller69"
            />
            <input autofocus id="stgNameDivBTN" type="button" value="Ändern" />
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
