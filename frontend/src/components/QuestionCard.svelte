<main>
    <div id="body">
        <div id="topDivs">
            <div id="topQuestion"><p>Frage 25</p></div>
            <div id="topDifficulty" class="topDifficultyEasy">
                <p id="difficulty">Einfach</p>
            </div>
        </div>
        <p id="questionTxt">Wer ist eine Taube?</p>
    </div>
</main>

<style>
    #body {
        padding: 2em;

        background-image: -webkit-linear-gradient(
            -45deg,
            rgba(142, 114, 222, 1) 0%,
            rgba(55, 71, 212, 1) 100%
        );

        border-radius: 3em;

        box-shadow: 0px 4px 32.8px #727dde;
    }

    #topDivs {
        display: flex;
        align-items: center;
        margin-top: -3.5em;
    }

    #topQuestion {
        border-radius: 50px;
        padding: 1em;
        padding-left: 1.4em;
        padding-right: 1.4em;

        box-shadow: 0px 4px 17.8px rgba(0, 0, 0, 0.25);

        background-color: rgba(0, 0, 0, 0.53);
        -webkit-backdrop-filter: blur(20.3px) contrast(100%);
    }

    #topQuestion p {
        color: white;
    }

    #questionTxt {
        color: white;
        margin-top: 1em;
        font-size: larger;
    }

    #difficulty {
        color: white;
        font-size: small;
    }

    #topDifficulty {
        margin-left: 1em;

        border-radius: 50px;
        padding: 0.8em;
        padding-left: 1.2em;
        padding-right: 1.2em;

        box-shadow: 0px 4px 17.8px rgba(0, 0, 0, 0.25);

        backdrop-filter: blur(20.3px) contrast(100%);
        -webkit-backdrop-filter: blur(20.3px) contrast(100%);
    }

    .topDifficultyEasy {
        background-color: rgba(18, 146, 46, 0.53);
    }
</style>
