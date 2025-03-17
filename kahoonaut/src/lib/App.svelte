<script lang="ts">
  import { writable } from "svelte/store";
  import QuestionList from "./components/QuestionList.svelte";
  import Toast from "./components/Toast.svelte";
  import type { Question } from "$lib/types";
	import PromptInputComponent from "./components/PromptInput.svelte";

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

<div class="container d-flex justify-content-center align-items-center vh-100 gap-2">
  <div class="card p-4 shadow" style="width: 600px;">
    <PromptInputComponent {questions} {showToast} />

  </div>

  <div class="card p-4 shadow" style="width: 600px;">
    <QuestionList {questions} />
  </div>

    {#if $toastMessage}
      <Toast
        message={$toastMessage}
        isSuccess={$isSuccess}
        onClose={closeToast}
      />
    {/if}
</div>
