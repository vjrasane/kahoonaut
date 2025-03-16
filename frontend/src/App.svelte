<script lang="ts">
  import { writable } from "svelte/store";
  import type { Question } from "./types";
  import PromptInput from "./components/PromptInput.svelte";
  import QuestionList from "./components/QuestionList.svelte";
  import Toast from "./components/Toast.svelte";

  let questions = writable<Question[]>([]);
  let toastMessage = writable<string | null>(null);
  let isSuccess = writable<boolean>(true);

  async function onSubmit(prompt: string) {
    try {
      const response = await fetch("/api/v1/prompt", {
        method: "POST",
        headers: { "Content-Type": "text/plain" },
        body: prompt,
      });
      const data: Question[] = await response.json();
      questions.set(data);
      toastMessage.set("Questions retrieved successfully!");
      isSuccess.set(true);
    } catch (error) {
      toastMessage.set("Failed to fetch questions.");
      isSuccess.set(false);
    }
  }

  // function onQuestionChange(
  //   modifier: (q: Question[]) => Question[],
  // ) {
  //   questions.update((qs) => qs.map((q, i) => (i === index ? modifier(q) : q)));
  // }

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
