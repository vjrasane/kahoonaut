<script lang="ts">
	import type { Question } from '$lib/types';
	import HamburgerMenu from './HamburgerMenu.svelte';
	import HamburgerMenuItem from './HamburgerMenuItem.svelte';

	export let question: Question;
	export let onRemove: () => void;
	export let onShuffle: () => void;
	export let onQuestionChange: (modifier: (question: Question) => Question) => void;

	function handleAnswerChange(event: Event, index: number) {
		const target = event.target as HTMLInputElement;
		onQuestionChange((question) => ({
			...question,
			answers: question.answers.map((answer, i) => (i === index ? target.value : answer))
		}));
	}
	function handleQuestionChange(event: Event) {
		const target = event.target as HTMLInputElement;
		onQuestionChange((question) => ({
			...question,
			question: target.value
		}));
	}

	function handleCorrectAnswerChange(event: Event, index: number) {
		const target = event.target as HTMLInputElement;
		onQuestionChange((question) => ({
			...question,
			correct: target.checked
				? [...question.correct, index]
				: question.correct.filter((k) => k !== index)
		}));
	}

    $: hasAnswers = question.answers.filter(a => !!a).length > 1;
</script>

<div class="flex flex-col items-center gap-2 px-1">
	<div class="w-full flex gap-1">
		<input
			type="text"
			class="input w-full"
			bind:value={question.question}
			on:input={handleQuestionChange}
		/>
		<HamburgerMenu>
			<HamburgerMenuItem onClick={onShuffle} disabled={!hasAnswers}>Shuffle answers</HamburgerMenuItem>
			<HamburgerMenuItem onClick={onRemove}>Remove</HamburgerMenuItem>
		</HamburgerMenu>
	</div>

	<div class="grid grid-cols-2 gap-2">
		{#each question.answers as answer, index}
			<div class="flex gap-1">
				<input
					type="text"
					class="input"
					bind:value={answer}
					on:input={(e) => handleAnswerChange(e, index)}
				/>
				<input
					type="checkbox"
					class="form-check-input"
					checked={question.correct.includes(index)}
					on:change={(e) => handleCorrectAnswerChange(e, index)}
				/>
			</div>
		{/each}
	</div>
</div>
