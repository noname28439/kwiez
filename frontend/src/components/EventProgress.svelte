<script>
    export let topPlayer = 0;
    export let ownPlayer = 0;
    export let amountQuestions = 0;



</script>

<main>
    <progress id="progressTop" max={amountQuestions} value={topPlayer}>{topPlayer}%</progress>
    <progress id="progressPlayer" max={amountQuestions} value={ownPlayer}>{ownPlayer}%</progress>
</main>

<style>
    #progressTop {
        border-radius: 7px;
        width: 100%;
        height: 0.4em;
    }
    #progressTop::-webkit-progress-bar {
        background-color: var(--background-color);
        border-radius: 7px;
    }
    #progressTop::-webkit-progress-value {
        /*background-color: #5b59d5;*/
        background-image: -webkit-linear-gradient(
            0deg,
            var(--linearGradient-purple1) 0%,
            var(--linearGradient-purple2) 100%
        );
        border-radius: 7px;
        box-shadow: 0px 1px 4px rgba(91, 89, 213, 0.5);
    }

    #progressPlayer {
        border-radius: 7px;
        width: 100%;
        height: 1em;
    }
    #progressPlayer::-webkit-progress-bar {
        background-color: var(--background-color);
        border-radius: 7px;
    }
    #progressPlayer::-webkit-progress-value {
        /*background-color: #d98651;*/
        background-image: -webkit-linear-gradient(
            0deg,
            var(--linearGradient-orange1) 0%,
            var(--linearGradient-orange2) 100%
        );

        border-radius: 7px;
        box-shadow: 0px 1px 10px rgba(217, 134, 81, 0.5);
    }
</style>
