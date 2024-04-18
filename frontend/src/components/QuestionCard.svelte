<script>
    import {useEndpoint} from "../endpoints.js";

    import {cq, stats} from "./stores.js";
    import {sync} from "./networking.js";

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
    let blocked = false;


    async function correctAnswer() {
        correct = true;
        await new Promise((r) => setTimeout(r, 1000));
        correct = null;
    }

    async function wrongAnswer() {
        correct = false;
        await new Promise((r) => setTimeout(r, 500));
        correct = null;
    }

    //-----------------

    //timeout decrementer
    let intervalID = setInterval(()=>{
        if(current_timeout>0)
        current_timeout--;
    }, 1000);

    async function submitAnswer() {
        if(current_timeout>0) return;
        const res = await useEndpoint("answer", {answer: answer});

        console.log(res)

        if(res.error === "blocked") {
            current_timeout = Number(res.timeout);
            return;
        }

        if (res.correct) {
            document.getElementById("textInputFieldAnswer").value = "";
            correctAnswer();
            await sync();
        } else {
            await wrongAnswer();
        }
    }

    let current_timeout = 0;

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
                    class={question.schwierigkeit === "Leicht"
                    ? "topDifficultyEasy"
                    : question.schwierigkeit === "Mittel"
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

            <button type="submit" id="submitBtn" class={current_timeout>0>0?"blocked":""}>{current_timeout>0?`WARTE NOCH ${current_timeout} SEKUNDEN`:"OK"}</button>
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
        animation: flashWrong 0.5s forwards;
    }

    @keyframes flashWrong {
        0% {
            outline: 0.5em solid #ff0000;
            transform: perspective(800px) rotateY(0deg);
        }
        25% {
            transform: perspective(800px) rotateY(-15deg);
        }

        50% {
            transform: perspective(800px) rotateY(15deg);
            outline: 0.5em solid #ff0000;
        }

        75% {
            transform: perspective(800px) rotateY(-15deg);
        }

        100% {
            transform: perspective(800px) rotateY(0deg);
            outline: none;
        }
    }

    .correct {
        animation: flashCorrect 1s forwards;

    }

    @keyframes flashCorrect {
        0% {
            outline: 0.5em solid #1aff00;
            transform: perspective(800px) rotateY(0deg);

        }
        100% {
            outline: none;
            transform: perspective(800px) rotateY(360deg);
        }
    }

    .blocked {
        animation: flashBlocked 500ms forwards;
    }

    @keyframes flashBlocked {
        0% {
            transform: perspective(800px) rotateY(0deg);
            background-color: rgba(255, 0 , 0, 0.54);
        }


        25% {
            transform: translateY(0.5em);
        }

        50% {

            background-color: rgba(255, 0 , 0, 0.54);
        }

        100% {

        }
    }

</style>
