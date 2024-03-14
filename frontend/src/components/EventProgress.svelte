<main>
    <progress id="progressTop" max="100" value="80">70%</progress>
    <progress id="progressPlayer" max="100" value="70">70%</progress>
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
            rgba(142, 114, 222, 1) 0%,
            rgba(55, 71, 212, 1) 100%
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
            rgba(222, 185, 114, 1) 0%,
            rgba(212, 93, 55, 1) 100%
        );

        border-radius: 7px;
        box-shadow: 0px 1px 10px rgba(217, 134, 81, 0.5);
    }
</style>
