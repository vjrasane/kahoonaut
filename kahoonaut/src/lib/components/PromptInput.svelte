<script lang="ts">
	import type { PromptInput, Question } from '$lib/types';
	import { writable, type Writable } from 'svelte/store';
	export let questions: Writable<Question[]>;
	export let showToast: (message: string, success: boolean) => void;
	let isLoading = writable(false);

	async function onSubmit(prompt: string) {
		try {
			isLoading.set(true);
			const input: PromptInput = {
				prompt,
				excludeQuestions: $questions,
				language: 'finnish'
			};
			const response = await fetch('/api/v1/prompt', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify(input)
			});
			const data: Question[] = await response.json();
			questions.update((prev) => [...prev, ...data]);
			showToast('Questions retrieved successfully!', true);
		} catch (error) {
			showToast('Failed to fetch questions.', false);
		} finally {
			isLoading.set(false);
		}
	}
	let prompt = '';
</script>

<form on:submit={() => onSubmit(prompt)} class="w-full h-full flex flex-col py-4 pl-4 gap-2">
	<textarea
		on:keydown={(e) => {
			if (e.key !== 'Enter') return;
			e.preventDefault();
			onSubmit(prompt);
		}}
		class="textarea w-full flex-grow"
		rows="5"
		placeholder="Enter your prompt here..."
		bind:value={prompt}
	></textarea>
	<div class="flex gap-2">
		<button type="submit" class="btn btn-primary flex-grow">
			{#if $isLoading}
				<span class="spinner-border spinner-border-sm" role="status" aria-hidden="true"></span>
			{/if}
			Submit
		</button>
		<button
			type="button"
			class="btn btn-soft flex-grow"
			on:click={() => {
				prompt = '';
			}}>Clear</button
		>
	</div>
</form>
