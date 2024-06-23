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
            <p>Laden</p>
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
</main>

<style>
    main {
        margin: 1em 1em 0;
    }

    #EventProgress {
        margin-top: 2em;
    }

    #Content {
        margin-top: 3em;
        padding-bottom: 10em;
        word-break: break-word;
    }

    @media (min-width: 640px) {
        main {
            margin: 1em 0 0;
            width: 100%;
            justify-content: center;
        }

        #wrapper {
            max-width: 640px;
            margin-left: auto;
            margin-right: auto;
        }
    }

</style>
