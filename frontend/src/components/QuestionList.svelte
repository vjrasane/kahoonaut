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

    function shuffleAnswers(question: Question) {
        const keys = Object.keys(question.answers);
        const answers = Object.entries(question.answers).map(
            ([key, value]) => ({
                value,
                correct: question.correct.includes(key),
            }),
        );

        for (let i = answers.length - 1; i > 0; i--) {
            const j = Math.floor(Math.random() * (i + 1));
            [answers[i], answers[j]] = [answers[j], answers[i]];
        }

        const shuffledAnswers = answers.reduce(
            (
                acc: Record<string, { value: string; correct: boolean }>,
                answer,
                i,
            ) => {
                acc[keys[i]] = answer;
                return acc;
            },
            {},
        );

        const correct = Object.entries(shuffledAnswers)
            .filter(([key, answer]) => answer.correct)
            .map(([key]) => key);

        return {
            ...question,
            answers: Object.entries(shuffledAnswers).reduce(
                (acc: Record<string, string>, [key, { value }]) => ({
                    ...acc,
                    [key]: value,
                }),
                {},
            ),
            correct,
        };
    }

    function onShuffleAnswers(index: number) {
        questions.update((qs) => {
            return qs.map((q, i) => (i === index ? shuffleAnswers(q) : q));
        });
    }
</script>

<div class="mt-4">
    <h4 class="text-center text-primary">Generated Questions</h4>
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
