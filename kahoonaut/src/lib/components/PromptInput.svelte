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

<form on:submit={() => onSubmit(prompt)}>
	<textarea
		on:keydown={(e) => {
			if (e.key !== 'Enter') return;
			e.preventDefault();
			onSubmit(prompt);
		}}
		class="form-control mb-3"
		rows="5"
		placeholder="Enter your prompt here..."
		bind:value={prompt}
	></textarea>
	<div class="mt-3 gap-1 d-flex justify-start">
		<button type="submit" class="btn btn-primary w-100">
			{#if $isLoading}
				<span class="spinner-border spinner-border-sm" role="status" aria-hidden="true"></span>
			{/if}
			Submit
		</button>
		<button
			type="button"
			class="btn btn-secondary w-100"
			on:click={() => {
				prompt = '';
			}}>Clear</button
		>
	</div>
</form>
