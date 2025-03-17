<script lang="ts">
    import type { Question } from "$lib/types";

    export let question: Question;
    export let onRemove: () => void;
    export let onShuffle: () => void;
    export let onQuestionChange: (
        modifier: (question: Question) => Question,
    ) => void;

    function handleAnswerChange(event: Event, index: number) {
        const target = event.target as HTMLInputElement;
        onQuestionChange((question) => ({
            ...question,
            answers: question.answers.map((answer, i) =>
                i === index ? target.value : answer,
            ),
        }));
    }
    function handleQuestionChange(event: Event) {
        const target = event.target as HTMLInputElement;
        onQuestionChange((question) => ({
            ...question,
            question: target.value,
        }));
    }

    function handleCorrectAnswerChange(event: Event, index: number) {
        const target = event.target as HTMLInputElement;
        onQuestionChange((question) => ({
            ...question,
            correct: target.checked
                ? [...question.correct, index]
                : question.correct.filter((k) => k !== index),
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
        {#each question.answers as answer, index}
            <div class="col-6 d-flex align-items-center">
                <span class="me-2 fw-bold">{index + 1}.</span>
                <!-- Answer label -->
                <input
                    type="text"
                    class="form-control me-2"
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
    <div class="mt-3 gap-1 d-flex justify-start">
        <button class="btn btn-danger" on:click={onRemove}>🗑 Remove</button>
        <button class="btn btn-secondary" on:click={onShuffle}
            >🔀 Shuffle Answers</button
        >
    </div>
</div>
