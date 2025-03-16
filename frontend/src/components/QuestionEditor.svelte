<script lang="ts">
    import type { Question } from "../types";

    export let question: Question;
    export let onRemove: () => void;
    export let onShuffle: () => void;
    export let onQuestionChange: (
        modifier: (question: Question) => Question,
    ) => void;

    function handleAnswerChange(event: Event, key: string) {
        const target = event.target as HTMLInputElement;
        onQuestionChange((question) => ({
            ...question,
            answers: { ...question.answers, [key]: target.value },
        }));
    }
    function handleQuestionChange(event: Event) {
        const target = event.target as HTMLInputElement;
        onQuestionChange((question) => ({
            ...question,
            question: target.value,
        }));
    }

    function handleCorrectAnswerChange(event: Event, key: string) {
        const target = event.target as HTMLInputElement;
        onQuestionChange((question) => ({
            ...question,
            correct: target.checked
                ? [...question.correct, key]
                : question.correct.filter((k) => k !== key),
        }));
    }
</script>

<div class="card mb-3 p-3 shadow-sm" style="max-width: 600px;">
    <input
        type="text"
        class="form-control mb-2 fw-bold"
        bind:value={question.question}
        on:input={handleQuestionChange}
    />

    <div class="row g-2 mt-2">
        {#each Object.entries(question.answers) as [key, value]}
            <div class="col-6 d-flex align-items-center">
                <span class="me-2 fw-bold">{key}.</span>
                <!-- Answer label -->
                <input
                    type="text"
                    class="form-control me-2"
                    bind:value={question.answers[key]}
                    on:input={(e) => handleAnswerChange(e, key)}
                />
                <input
                    type="checkbox"
                    class="form-check-input"
                    checked={question.correct.includes(key)}
                    on:change={(e) => handleCorrectAnswerChange(e, key)}
                />
            </div>
        {/each}
    </div>
    <div class="mt-3 gap-1 d-flex justify-start">
        <button class="btn btn-danger" on:click={onRemove}>🗑 Remove</button>
        <button class="btn btn-secondary" on:click={onShuffle}
            >🔀 Shuffle Answers</button
        >
    </div>
</div>
