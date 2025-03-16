<script lang="ts">
  import { writable } from "svelte/store";
  import PromptInput from "./components/PromptInput.svelte";
  import QuestionList from "./components/QuestionList.svelte";
  import Toast from "./components/Toast.svelte";
  import type { Question } from "./types";

  let questions = writable<Question[]>([]);
  let toastMessage = writable<string | null>(null);
  let isSuccess = writable<boolean>(true);

  async function onSubmit(prompt: string) {
    try {
      const response = await fetch("/api/v1/prompt", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ prompt, 
          questions: $questions.map(q => q.question)
         }),
      });
      const data: Question[] = await response.json();
      questions.update(prev => [ ...prev, ...data ]);
      toastMessage.set("Questions retrieved successfully!");
      isSuccess.set(true);
    } catch (error) {
      toastMessage.set("Failed to fetch questions.");
      isSuccess.set(false);
    }
  }

  function closeToast() {
    toastMessage.set(null);
  }
</script>

<div class="container d-flex justify-content-center align-items-center vh-100">
  <div class="card p-4 shadow" style="width: 600px;">
    <PromptInput {onSubmit} />
    <QuestionList {questions} />
    {#if $toastMessage}
      <Toast
        message={$toastMessage}
        isSuccess={$isSuccess}
        onClose={closeToast}
      />
    {/if}
  </div>
</div>
