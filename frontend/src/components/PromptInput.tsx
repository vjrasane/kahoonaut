import { createSignal } from "solid-js";

export function PromptInput(props: {
  onSubmit: (prompt: string) => void;
}) {
  const [prompt, setPrompt] = createSignal("");

  return (
    <div>
      <textarea
        class="form-control mb-3"
        rows={5}
        placeholder="Enter your prompt here..."
        value={prompt()}
        onInput={(e) => setPrompt(e.target.value)}
      />
      <button
        class="btn btn-primary w-100"
        onClick={() => props.onSubmit(prompt())}
      >
        Submit
      </button>
    </div>
  );
}
