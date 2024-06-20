<script>
    import {stats} from "./stores.js";
    import {tweened} from "svelte/motion";
    import {cubicOut} from "svelte/easing";
    import {onMount} from "svelte";

    let amountQuestions = 0;

    const ownPlayer = tweened(0, {
        duration: 400,
        easing: cubicOut
    });
    const topPlayer = tweened(0, {
        duration: 400,
        easing: cubicOut
    });








    stats.subscribe((value) => {
        ownPlayer.set(value.progress);
        topPlayer.set(value.top_progress);
        amountQuestions = value.count;
    })


    window.setProgressOwn = (number) => {
        ownPlayer.set(number);
    }
    window.setProgressTop = (number) => {
        topPlayer.set(number);
    }



    //Disable progress text overflow

    let progressValuePlayerTxt
    let progressValueTopTxt

    let progressBarPlayer
    let progressBarTop

    let progressPlayerPosition
    let progressTopPosition

    onMount(async () => {


        ownPlayer.subscribe(value => {

            progressPlayerPosition = getPosition(progressValuePlayerTxt, progressBarPlayer, value)

        })

        topPlayer.subscribe(value => {

            progressTopPosition = getPosition(progressValueTopTxt, progressBarTop, value)

        })



    });

    function getPosition(progressValueTxt, progressBar, value) {

        if(progressValueTxt.clientWidth) {
            //get text width
            let txt_Width = (progressValueTxt.clientWidth + 1)

            //get progress width
            let bar_Width = (progressBar.clientWidth + 1)
            let percentage = (value / amountQuestions);
            let barProgressWidth = bar_Width * percentage

            //set position
            if( (txt_Width / 2) > barProgressWidth) {
                return 0

            } else if( (bar_Width - (txt_Width / 2)) < barProgressWidth) {
                return 2

            } else {
                return 1
            }
        }

    }






</script>

<main>

    <div id="progressTopDiv">

        <div style="margin-bottom: -0.8em">

        <div class="progressValue" style={progressTopPosition === 1 ? `left: ${($topPlayer / amountQuestions) * 100}%` : ""}>
            <div class={progressTopPosition !== 1 ? progressTopPosition === 0 ? "progressValueBegin" : "progressValueEnd" : "progressValueCenter"}>
                <p bind:this={progressValueTopTxt}
                   style="font-size: 13px; color: var(--linearGradient-purple2); white-space: nowrap;">
                    Top Spieler {Math.floor(($topPlayer / amountQuestions) * 100)}%</p>
            </div>

        </div>

        <div class="progressValue" style="left: {($topPlayer / amountQuestions) * 100}%">

            <div class="progressValueCenter">
                <ion-icon name="caret-down" style="color: var(--linearGradient-purple2);"></ion-icon>
            </div>

        </div>

        </div>


        <progress bind:this={progressBarTop} id="progressTop" max={amountQuestions} value={$topPlayer}>{topPlayer}%</progress>
    </div>





    <div id="progressPlayerDiv">
        <progress bind:this={progressBarPlayer} id="progressPlayer" max={amountQuestions} value={$ownPlayer}>{ownPlayer}%</progress>

        <div class="progressValue" style="left: {($ownPlayer / amountQuestions) * 100}%">

            <div class="progressValueCenter">
                <ion-icon name="caret-up" style="color: var(--linearGradient-orange2);"></ion-icon>
            </div>

        </div>

        <div class="progressValue" style={progressPlayerPosition === 1 ? `left: ${($ownPlayer / amountQuestions) * 100}%` : ""}>

            <div class={progressPlayerPosition !== 1 ? progressPlayerPosition === 0 ? "progressValueBegin" : "progressValueEnd" : "progressValueCenter"}>
                <p bind:this={progressValuePlayerTxt}
                   style="font-size: 13px; color: var(--linearGradient-orange2); white-space: nowrap;">
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


    .progressValue {
        position: relative;
    }

    .progressValueBegin {
        display: flex;
        flex-direction: column;
        align-items: start;
    }
    .progressValueEnd {
        display: flex;
        flex-direction: column;
        align-items: end;
    }

    .progressValueCenter {
        transform: translateX(-50%);
        display: flex;
        align-items: center;
        flex-direction: column;
    }

</style>
