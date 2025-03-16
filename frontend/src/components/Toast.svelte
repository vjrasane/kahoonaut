<script lang="ts">
    export let message: string;
    export let isSuccess: boolean;
    export let onClose: () => void;
</script>

<div
    class="toast show position-fixed bottom-0 end-0 m-3 {isSuccess
        ? 'bg-success text-white'
        : 'bg-danger text-white'}"
    role="alert"
>
    <div class="toast-body">
        {message}
        <button
            type="button"
            class="btn-close btn-close-white ms-2 me-2"
            aria-label="Close"
            on:click={onClose}
        ></button>
    </div>
</div>
