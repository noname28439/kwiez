<script>
    import {stats} from "./stores.js";
    import {tweened} from "svelte/motion";
    import {cubicOut} from "svelte/easing";

    let amountQuestions = 0;

    const ownPlayer = tweened(0, {
        duration: 400,
        easing: cubicOut
    });
    const topPlayer = tweened(0, {
        duration: 400,
        easing: cubicOut
    });

    //let progressPercentage = (ownPlayer / amountQuestions) * 100;


    stats.subscribe((value) => {
        ownPlayer.set(value.progress);
        topPlayer.set(value.top_progress);
        amountQuestions = value.count;
    })


    window.setProgress = (number) => {
        ownPlayer.set(number);
    }


</script>

<main>

    <div id="progressTopDiv">
        <div id="progressValueTop" style="margin-bottom: -1em; left: {($topPlayer / amountQuestions) * 100}%">

            <div style="left: 50%;transform: translateX(-50%); text-align: center">
                <p style="font-size: 13px; color: var(--linearGradient-purple2); white-space: nowrap;">
                    Top Spieler {Math.floor(($topPlayer / amountQuestions) * 100)}%</p>
                <ion-icon name="caret-down" style="color: var(--linearGradient-purple2);"></ion-icon>

            </div>

        </div>
        <progress id="progressTop" max={amountQuestions} value={$topPlayer}>{topPlayer}%</progress>
    </div>

    <div id="progressPlayerDiv">
        <progress id="progressPlayer" max={amountQuestions} value={$ownPlayer}>{ownPlayer}%</progress>
        <div id="progressValuePlayer" style="left: {($ownPlayer / amountQuestions) * 100}%">

            <div style="left: 50%;transform: translateX(-50%); text-align: center">
                <ion-icon name="caret-up" style="color: var(--linearGradient-orange2);"></ion-icon>
                <p style="font-size: 13px; color: var(--linearGradient-orange2); white-space: nowrap;">
                    Du {Math.floor(($ownPlayer / amountQuestions) * 100)}%</p>
            </div>

        </div>

    </div>
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


    #progressPlayerDiv, #progressTopDiv {
        position: relative;
        text-align: center;
        overflow: hidden;
    }

    #progressValuePlayer, #progressValueTop {
        position: relative;
    }

</style>
