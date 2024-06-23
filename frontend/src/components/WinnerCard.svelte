<script>
    import WinnerImg from "../../public/images/WinnerImg.svelte"

    import {stats, ranking} from "./stores.js";
    import {authToken} from "../authentication.js";


    let information;
    let rankingData;

    stats.subscribe((value) => {
        information = value;

    })
    ranking.subscribe((value) => {
        rankingData = value;
    })


</script>

<main>
    <div id="imageWrapper">
        <WinnerImg height="25em" width="25em" />
    </div>

    <div id="textbox">
        <h1>Herzlichen Glückwunsch! 🥳</h1>
        <h2>Du hast alle {information.count-1} Fragen beantwortet.</h2>
        <p id="lowText">{information.nickname}_{authToken}</p>
    </div>


</main>

<style>

    main {

    }

    #imageWrapper {
        margin-top: -3em;
        display: flex;
        justify-content: center;
    }

    #imageCard {
        height: 30px;
    }

    #textbox {
        text-align: center;
    }

    #lowText {
        margin-top: 2em;
        color: lightgray;
        font-size: 0.8em;
    }



</style>