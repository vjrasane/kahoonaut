<script lang="ts">
	import type { Question } from '$lib/types';
	import { writable } from 'svelte/store';
	import PromptInputComponent from './components/PromptInput.svelte';
	import QuestionList from './components/QuestionList.svelte';
	import Toast from './components/Toast.svelte';

	let questions = writable<Question[]>([]);
	let toastMessage = writable<string | null>(null);
	let isSuccess = writable<boolean>(true);

	function showToast(message: string, success: boolean) {
		toastMessage.set(message);
		isSuccess.set(success);
	}

	function closeToast() {
		toastMessage.set(null);
	}
</script>

<div class="h-screen w-screen flex justify-center items-center">
	<div class="grow-2 basis-0 h-full">
		<PromptInputComponent {questions} {showToast} />
	</div>
	<div class="divider divider-horizontal"></div>
	<div class="grow-2 basis-0 h-full">
		<QuestionList {questions} />
	</div>

	{#if $toastMessage}
		<Toast message={$toastMessage} isSuccess={$isSuccess} onClose={closeToast} />
	{/if}
</div>
