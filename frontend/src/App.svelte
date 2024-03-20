<script>
	import Header from "./components/Header.svelte";
	import EventProgress from "./components/EventProgress.svelte";
	import QuestionCard from "./components/QuestionCard.svelte";
	import {useEndpoint} from "./endpoints.js";

	window.uEP = useEndpoint;	// for debugging
</script>

<main>
	<div id="wrapper">
		<Header />
		<div id="EventProgress">
			<EventProgress />
		</div>
		<div id="QuestionCard">
			<QuestionCard />
		</div>
	</div>
</main>

<style>
	main {
		margin: 0 1em;
		margin-top: 1em;
	}

	#EventProgress {
		margin-top: 2em;
	}

	#QuestionCard {
		margin-top: 2em;
	}

	@media (min-width: 640px) {
		main {
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
