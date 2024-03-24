<script>
    import Header from "./components/Header.svelte";
    import EventProgress from "./components/EventProgress.svelte";
    import QuestionCard from "./components/QuestionCard.svelte";
    import {useEndpoint} from "./endpoints.js";
    import {onMount} from "svelte";

    window.uEP = useEndpoint;	// for debugging

    let stats = null;
    let ranking = null;

    onMount(async () => {

        stats = await useEndpoint("stats", {});
        ranking = await useEndpoint("ranking", {})
    })

</script>

<main>
    <div id="wrapper">

        {#if stats}
            <Header stats={stats} ranking={ranking}/>
        {:else}
            <Header playerName={""} ranking={[]}/>
        {/if}


        <div id="EventProgress">

            {#if stats}
                <EventProgress
                        amountQuestions={stats.count}
                        ownPlayer={stats.progress}
                        topPlayer={stats.top_progress}
                />
            {:else}
                <EventProgress
                        amountQuestions={0}
                        ownPlayer={0}
                        topPlayer={0}
                />
            {/if}

        </div>
        <div id="QuestionCard">
            <QuestionCard questionNumber={stats}/>
        </div>
    </div>
</main>

<style>
    main {
        margin: 0 1em;
        margin-top: 1em;
    }

    #EventProgress {
        margin-top: 2em;
    }

    #QuestionCard {
        margin-top: 2em;
    }

    @media (min-width: 640px) {
        main {
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
