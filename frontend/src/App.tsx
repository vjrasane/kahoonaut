import { createSignal } from "solid-js";
import { Question } from "./components/Question";
import { PromptInput } from "./components/PromptInput";
import { QuestionsList } from "./components/QuestionList";
import Toast from "./components/Toast";

export default function App() {
  const [questions, setQuestions] = createSignal<Question[]>([]);
  const [toastMessage, setToastMessage] = createSignal<string | null>(null);
  const [isSuccess, setIsSuccess] = createSignal<boolean>(true);

  const onSubmit = async (prompt: string) => {
    try {
      const response = await fetch("/api/v1/prompt", {
        method: "POST",
        headers: { "Content-Type": "text/plain" },
        body: prompt,
      });
      const data: Question[] = await response.json();
      setQuestions(data);
      setToastMessage("Questions retrieved successfully!");
      setIsSuccess(true);
    } catch (error) {
      setToastMessage("Failed to fetch questions: " + String(error));
      setIsSuccess(false);
    }
  };

  const onAnswerChange = (index: number, key: string, newValue: string) => {
    setQuestions(
      questions().map((question, i) => i === index ? {
        ...question,
        answers: {
          ...question.answers,
          [key]: newValue,
        },
      } : question)
    );
  };

  return (
    <div class="container d-flex justify-content-center align-items-center vh-100">
      <div class="card p-4 shadow" style={{ width: "600px" }}>
        <PromptInput onSubmit={onSubmit} />
        <QuestionsList questions={questions()} onAnswerChange={onAnswerChange} />
        {toastMessage() && (
          <Toast
            message={toastMessage()!}
            isSuccess={isSuccess()}
            onClose={() => setToastMessage(null)}
          />
        )}
      </div>
    </div>
  );
}
