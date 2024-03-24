<script>

    import {onMount} from "svelte";
    import {useEndpoint} from "../endpoints.js";

    export let questionNumber;
    let question = null;

    let wrongAnswer = false;

    let rightAnswer = false;


    onMount(async () => {
        question = await useEndpoint("cq", {});
        console.log(question)
    });

    async function submitAnswer() {

        const answer = document.getElementById('textInputFieldAnswer').value

        const res = await useEndpoint("answer", {answer: answer});

        if (res.correct) {
            rightAnswer = true;
            alert("Richtig!")
            location.reload()
        } else {
            wrongAnswer = true;
            alert("Schlecht.")
        }


        console.log(res)
    }


</script>

<main>
    <div id="body">
        <div id="topDivs">
            <div id="topQuestion"><p>Frage
                {#if questionNumber}{questionNumber.progress + 1}{/if}
            </p></div>
            {#if question}

                {#if question.schwierigkeit == "Leicht"}
                    <div id="topDifficulty" class="topDifficultyEasy">
                        <p id="difficulty">

                            {question.schwierigkeit}

                        </p>
                    </div>
                {:else if question.schwierigkeit == "Mittel"}

                    <div id="topDifficulty" class="topDifficultyMiddle">
                        <p id="difficulty">

                            {question.schwierigkeit}

                        </p>
                    </div>
                {:else if question.schwierigkeit == "Schwer"}

                    <div id="topDifficulty" class={"topDifficultyHard"}>
                        <p id="difficulty">

                            {question.schwierigkeit}

                        </p>
                    </div>




            {/if}


            {/if}
        </div>
        <p id="questionTxt">

            {#if question}
                {question.frage}
            {/if}

        </p>

        <textarea id="textInputFieldAnswer" placeholder="Antwort hier eingeben..."/>

        <button id="submitBtn" on:click={submitAnswer}>OK</button>

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

    textarea {
        margin-top: 1em;
        font-weight: 800;
        font-family: "Manrope", sans-serif;
        width: 100%;
        border-radius: 1em;
        height: 200px;
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
</style>
