<script>
    import { useEndpoint } from "../endpoints.js";

    import { cq, stats } from "./stores.js";
    import { sync } from "./networking.js";

    let question = null;
    let questionNumber = null;

    //Rerender
    cq.subscribe((value) => {
        question = value;
    });

    stats.subscribe((value) => {
        questionNumber = value.progress;
    });
    //----------

    let answer = "";
    let correct = null;

    async function submitAnswer() {
        const res = await useEndpoint("answer", { answer: answer });
        grow();

        correct = res.correct;

        if (correct) {
            //location.reload()
            await new Promise((r) => setTimeout(r, 1000));
            document.getElementById("textInputFieldAnswer").value = "";
            correct = null;
            await sync();
        }
    }

    function grow() {
        document.getElementById("body").classList.add("grow");
        setTimeout(() => document.getElementById("body").classList.remove("grow"), 1000);
    }
</script>

<main>
    <div
        id="body"
        class={correct !== null ? (correct ? "correct" : "wrong") : ""}
    >
        <div id="topDivs">
            <div id="topQuestion">
                <p>
                    Frage {questionNumber + 1}
                </p>
            </div>

            <div
                id="topDifficulty"
                class={question.schwierigkeit == "Leicht"
                    ? "topDifficultyEasy"
                    : question.schwierigkeit == "Mittel"
                      ? "topDifficultyMiddle"
                      : "topDifficultyHard"}
            >
                <p id="difficulty">
                    {question.schwierigkeit}
                </p>
            </div>
        </div>
        <p id="questionTxt">
            {question.frage}
        </p>

        <form on:submit|preventDefault={submitAnswer}>
            <input
                bind:value={answer}
                type="text"
                id="textInputFieldAnswer"
                placeholder="Antwort hier eingeben..."
            />

            <button type="submit" id="submitBtn"> OK</button>
        </form>
    </div>
</main>

<style>
    #body {
        padding: 2em;

        background-image: -webkit-linear-gradient(
            -45deg,
            var(--linearGradient-purple1) 0%,
            var(--linearGradient-purple2) 100%
        );

        border-radius: 3em;

        box-shadow: 0px 4px 32.8px #727dde;

        transition: all 200ms ease-in-out;
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

        backdrop-filter: blur(20.3px) contrast(100%);
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

    .topDifficultyMiddle {
        background-color: rgba(146, 144, 18, 0.53);
    }

    .topDifficultyHard {
        background-color: rgba(146, 18, 18, 0.53);
    }

    #textInputFieldAnswer {
        margin-top: 1em;
        margin-bottom: 1em;
        font-weight: 800;
        font-family: "Manrope", sans-serif;
        width: 100%;
        border-radius: 1em;
        /*height: 200px;*/
        padding: 1.2em;

        box-shadow: 0px 4px 0px rgba(0, 0, 0, 0.25);
    }

    #submitBtn {
        width: 100%;
        height: 3em;
        border-radius: 1em;
        font-weight: 800;
        font-family: "Manrope", sans-serif;
        color: white;
        background-color: rgba(0, 0, 0, 0.54);
        border-width: 0;
        box-shadow: 0px 4px 0px rgba(0, 0, 0, 0.25);
    }

    #submitBtn:hover {
        background-color: rgba(0, 0, 0, 0.74);
        /*box-shadow: 0px 0px 0px rgba(0, 0, 0, 0.25);
        transform: translate(0px, 4px);
        -webkit-transform: translate(0px, 4px);
        transition: 0.1s;*/
        cursor: pointer;
    }

    .wrong {
        animation: flashWrong 1s forwards;
    }

    @keyframes flashWrong {
        0% {
            outline: 0.5em solid #ff0000;
        }
        100% {
            outline: none;
        }
    }

    .correct {
        animation: flashCorrect 1s forwards;
    }

    @keyframes flashCorrect {
        0% {
            outline: 0.5em solid #1aff00;
        }
        100% {
            outline: none;
        }
    }

    @keyframes grow {
        0% {
            transform: scale(1);
        }
        50% {
            transform: scale(1.1);
        }
        100% {
            transform: scale(1);
        }
    }

    .grow {
        animation: grow 1s ease-in-out;
    }
</style>
