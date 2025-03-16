<script lang="ts">
    import type { Writable } from "svelte/store";
    import type { Question } from "../types";
    import QuestionEditor from "./QuestionEditor.svelte";

    export let questions: Writable<Question[]>;

    function onQuestionChange(
        index: number,
        modifier: (q: Question) => Question,
    ) {
        questions.update((qs) =>
            qs.map((q, i) => (i === index ? modifier(q) : q)),
        );
    }

    function onQuestionRemove(index: number) {
        questions.update((qs) => qs.filter((q, i) => i !== index));
    }

    function shuffleAnswers(question: Question): Question {
        const answers = question.answers.map((answer, index) => ({
            value: answer,
            correct: question.correct.includes(index),
        }));

        for (let i = answers.length - 1; i > 0; i--) {
            const j = Math.floor(Math.random() * (i + 1));
            [answers[i], answers[j]] = [answers[j], answers[i]];
        }

        const correct = answers.reduce(
            (acc: number[], { correct }, index) =>
                correct ? [...acc, index] : acc,
            [],
        );

        return {
            ...question,
            answers: answers.map(({ value }) => value),
            correct,
        };
    }

    function onShuffleAnswers(index: number) {
        questions.update((qs) => {
            return qs.map((q, i) => (i === index ? shuffleAnswers(q) : q));
        });
    }

    async function onExport() {
        const response = await fetch("/api/v1/export", {
            method: "POST",
            headers: { "Content-Type": "text/plain" },
            body: JSON.stringify($questions),
        });
        const data = await response.blob();
        const url = window.URL.createObjectURL(data);
        const a = document.createElement("a");
        a.href = url;
        a.download = "questions.xlsx";
        document.body.appendChild(a); // we need to append the element to the dom -> otherwise it will not work in firefox
        a.click();
        a.remove(); //afterwards we remove the element again
    }
</script>

<div class="mt-4">
    <h4 class="text-center text-primary">Generated Questions</h4>
    <div class="mt-3 gap-1 d-flex justify-start">
        <button class="btn btn-success w-100" on:click={onExport}>Export</button
        >
    </div>
    <hr />
    {#if $questions.length === 0}
        <p class="text-center text-muted">
            No questions available yet. Submit a prompt!
        </p>
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
