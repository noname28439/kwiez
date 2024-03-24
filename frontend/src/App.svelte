<script>
    import Header from "./components/Header.svelte";
    import EventProgress from "./components/EventProgress.svelte";
    import QuestionCard from "./components/QuestionCard.svelte";
    import {useEndpoint} from "./endpoints.js";
    import {onMount} from "svelte";
    import {sync} from "./components/networking.js";

    window.uEP = useEndpoint;	// for debugging

    const promise = sync();


</script>

<main>
    <div id="wrapper">

        {#await promise}
            <p>Laden</p>
        {:then {stats, ranking, cq}}

            {#if stats}
                <Header/>
            {:else}
                <Header />
            {/if}


            <div id="EventProgress">

                {#if stats}
                    <EventProgress/>
                {:else}
                    <EventProgress/>
                {/if}

            </div>
            <div id="QuestionCard">
                <QuestionCard/>
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

    #QuestionCard {
        margin-top: 2em;
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
