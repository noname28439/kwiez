<script>
    import {each} from "svelte/internal";

    export let rankingData;
    export let stats;

    const rankingDATA = {
        global: [
            {
                username: "Manfredkiller187",
                points: 45,
                rank: 1,
            },
            {
                username: "LeonBanause69",
                points: 45,
                rank: 2,
            },
            {
                username: "EverslohniumKek420",
                points: 45,
                rank: 3,
            },
        ],
        player: {
            username: "EverslohniumKek420",
            points: 45,
            rank: 248,
        },

    };
</script>

<main>
    <div id="wrapper">
        <div class="icon-div">
            <ion-icon name="trophy"></ion-icon>
        </div>

        <div>
            {#if rankingData}
                {#each rankingData as entry, index}
                    <div class="rankingEntry">
                        <div
                                style="display:flex; flex-direction: row; align-items: center;"
                        >
                            <p class="placeRankingTxt">{index + 1}.</p>
                            <p>{entry[0]}</p>
                        </div>

                        <p style="margin-left: 1em;">{entry[1]}</p>
                    </div>
                {/each}
            {/if}

            <div class="rankingEntryOwn">

                {#if stats}
                <div
                        style="display:flex; flex-direction: row; align-items: center;"
                >
                    <p class="placeRankingTxt">100.</p>
                    <p>{stats.nickname}</p>
                </div>

                <p style="margin-left: 1em;">{stats.progress}</p>

                    {/if}
            </div>
        </div>
    </div>
</main>

<style>
    p {
        font-weight: 800;
    }

    .icon-div {
        display: flex;
        width: 100%;
        flex-direction: row;
        align-items: center;
        justify-content: center;
    }

    .icon-div ion-icon {
        font-size: 5em;
        color: var(--linearGradient-orange1);
        filter: drop-shadow(0px 4px 0px var(--background-color));
        margin-bottom: 0.2em;
    }

    .rankingEntry {
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: space-between;
        margin-top: 1em;
        margin-bottom: 1em;
        padding: 1em;
        border-radius: 1.5em;
        background-color: var(--background-color2);
        box-shadow: 0px 4px 0px rgba(0, 0, 0, 0.1);

        border-style: solid;
        border-width: 0.15em;
        border-color: var(--background-color);
    }

    .rankingEntryOwn {
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: space-between;
        margin-top: 3em;
        margin-bottom: 1em;
        padding: 1em;
        border-radius: 1em;

        background-image: -webkit-linear-gradient(
                0deg,
                var(--linearGradient-orange1) 0%,
                var(--linearGradient-orange2) 200%
        );
        box-shadow: 0px 4px 32.8px var(--linearGradient-orange1);
    }

    .rankingEntryOwn p {
        color: white;
    }

    .placeRankingTxt {
        margin-right: 1em;
        font-size: 20px;
    }
</style>
