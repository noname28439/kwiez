<script>
    import {each} from "svelte/internal";

    import {stats, ranking} from "../stores.js";

    let information;
    let rankingData;

    stats.subscribe((value) => {
        information = value;

    })
    ranking.subscribe((value) => {
        rankingData = value;
    })

</script>

<main>
    <div id="wrapper">
        <div class="icon-div">
            <ion-icon name="trophy"></ion-icon>
        </div>

        <div>
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

            {#if information.nickname}

                <div class="rankingEntryOwn">


                    <div
                            style="display:flex; flex-direction: row; align-items: center;"
                    >
                        <p class="placeRankingTxt">{information.rank}.</p>
                        <p>{information.nickname}</p>
                    </div>

                    <p style="margin-left: 1em;">{information.progress}</p>

                </div>

            {:else}
                <p style="margin-top: 2em;">Lege in den Einstellungen einen Spielernamen fest, um ab Frage 2 dem
                    AEG-Ranking beitreten zu können.</p>
            {/if}

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
