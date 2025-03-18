<script lang="ts">
	import type { Writable } from 'svelte/store';
	import type { Question } from '../types';
	import QuestionEditor from './QuestionEditor.svelte';

	export let questions: Writable<Question[]>;

	function onQuestionChange(index: number, modifier: (q: Question) => Question) {
		questions.update((qs) => qs.map((q, i) => (i === index ? modifier(q) : q)));
	}

	function onQuestionRemove(index: number) {
		questions.update((qs) => qs.filter((q, i) => i !== index));
	}

	function shuffleAnswers(question: Question): Question {
		const answers = question.answers.map((answer, index) => ({
			value: answer,
			correct: question.correct.includes(index)
		}));

		for (let i = answers.length - 1; i > 0; i--) {
			const j = Math.floor(Math.random() * (i + 1));
			[answers[i], answers[j]] = [answers[j], answers[i]];
		}

		const correct = answers.reduce(
			(acc: number[], { correct }, index) => (correct ? [...acc, index] : acc),
			[]
		);

		return {
			...question,
			answers: answers.map(({ value }) => value),
			correct
		};
	}

	function onShuffleAnswers(index: number) {
		questions.update((qs) => {
			return qs.map((q, i) => (i === index ? shuffleAnswers(q) : q));
		});
	}

	async function onExport() {
		const response = await fetch('/api/v1/export', {
			method: 'POST',
			headers: { 'Content-Type': 'text/plain' },
			body: JSON.stringify($questions)
		});
		const data = await response.blob();
		const url = window.URL.createObjectURL(data);
		const a = document.createElement('a');
		a.href = url;
		a.download = 'questions.xlsx';
		document.body.appendChild(a); // we need to append the element to the dom -> otherwise it will not work in firefox
		a.click();
		a.remove(); //afterwards we remove the element again
	}

	async function onAdd() {
		questions.update((qs) => [...qs, { question: '', answers: ['', '', '', ''], correct: [] }]);
	}

	async function onShuffle() {
		questions.update((qs) => qs.map(shuffleAnswers));
	}
</script>

<div class="h-full w-full pr-4 py-4">
	<div class="flex gap-2 justify-center">
		<button class="btn btn-primary grow" on:click={onAdd}>Add</button>
		<button class="btn btn-secondary grow" disabled={!$questions.length} on:click={onShuffle}
			>Shuffle Answers</button
		>
		<button class="btn btn-accent grow" disabled={!$questions.length} on:click={onExport}>Export</button>
	</div>
	<div class="divider"></div>
	<div class="overflow-auto">
		{#if $questions.length === 0}
			<p class="text-center text-muted">No questions available yet. Submit a prompt!</p>
		{:else}
			{#each $questions as question, index}
				<QuestionEditor
					{question}
					onRemove={() => onQuestionRemove(index)}
					onShuffle={() => onShuffleAnswers(index)}
					onQuestionChange={(mod) => onQuestionChange(index, mod)}
				/>
			{/each}
		{/if}
	</div>
</div>
