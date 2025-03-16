<script lang="ts">
    export let onSubmit: (prompt: string) => void;
    let prompt = "";
</script>

<div>
    <textarea
        class="form-control mb-3"
        rows="5"
        placeholder="Enter your prompt here..."
        bind:value={prompt}
    ></textarea>
    <button class="btn btn-primary w-100" on:click={() => onSubmit(prompt)}
        >Submit</button
    >
</div>
