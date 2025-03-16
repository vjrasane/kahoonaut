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
    <div class="mt-3 gap-1 d-flex justify-start">
    <button class="btn btn-primary w-100" on:click={() => onSubmit(prompt)}
        >Submit</button
    >
    <button class="btn btn-secondary w-100" on:click={() => {
        prompt = "";
    }}
        >Clear</button
    >
    </div>
</div>
