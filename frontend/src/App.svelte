<script>
	import Header from "./components/Header.svelte";
	import EventProgress from "./components/EventProgress.svelte";
	import QuestionCard from "./components/QuestionCard.svelte";
</script>

<main>
	<Header />
	<div id="EventProgress">
		<EventProgress />
	</div>
	<div id="QuestionCard">
		<QuestionCard />
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
			max-width: none;
		}
	}
</style>
