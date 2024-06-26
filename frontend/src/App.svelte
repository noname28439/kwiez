<script>
    import Header from "./components/Header.svelte";
    import EventProgress from "./components/EventProgress.svelte";
    import QuestionCard from "./components/QuestionCard.svelte";
    import WinnerCard from "./components/WinnerCard.svelte";
    import {useEndpoint} from "./endpoints.js";
    import {fly} from "svelte/transition";
    import {sync} from "./components/networking.js";

    window.uEP = useEndpoint;	// for debugging

    const promise = sync();



</script>

<main>
    <div id="wrapper">
        {#await promise}
            <div class="loader"></div>
        {:then {stats, ranking, cq, statsRes}}

            {#if stats}
                <Header/>
            {:else}
                <Header/>
            {/if}


            <div id="EventProgress" transition:fly={{ y: 50 }}>

                {#if stats}
                    <EventProgress/>
                {:else}
                    <EventProgress/>
                {/if}

            </div>
            <div id="Content" transition:fly={{ y: 50 }}>
                {#if statsRes.count > statsRes.progress}
                    <QuestionCard />
                {:else}
                    <WinnerCard />

                {/if}
            </div>

        {/await}


    </div>
    <div class="footer">
        <p style="font-weight: 800; font-family: 'Manrope', sans-serif; color: #d5d5d5">powered by Odin</p>

    </div>
</main>

<style>

    #wrapper {
        padding: 1em 1em;
        overflow: hidden;
    }

    #EventProgress {
        margin-top: 1em;
    }

    #Content {
        margin-top: 3em;
        padding-bottom: 10em;
        word-break: break-word;
    }

    @media (min-width: 640px) {
        main {
            /*margin: 1em 0 0;*/
            width: 100%;
            justify-content: center;
        }

        #wrapper {
            max-width: 640px;
            margin-left: auto;
            margin-right: auto;

            padding: 1em 3em;
            overflow-x: hidden;
        }


    }

    .loader {
        border: 8px solid #f3f3f3; /* Light grey */
        border-top: 8px solid #acacac; /* Blue */
        border-radius: 50%;
        width: 40px;
        height: 40px;
        animation: spin 2s linear infinite;
        display: flex;
        margin-left: auto;
        margin-right: auto;
    }

    @keyframes spin {
        0% { transform: rotate(0deg); }
        100% { transform: rotate(360deg); }
    }


    .footer {

        width: 100%;
        position: absolute;
        bottom: 0;
        display: flex;
        justify-content: center;
        margin-bottom: 1em;

    }

</style>
