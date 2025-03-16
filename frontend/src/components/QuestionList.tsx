import { For, Index } from "solid-js";
import { Question, QuestionComponent } from "./Question";

export function QuestionsList(props: {
  questions: Question[];
  onAnswerChange: (index: number, key: string, newValue: string) => void;
}) {
  return (
    <div class="mt-4">
      <h4 class="text-center text-primary">Generated Questions</h4>
      <hr />
      {props.questions.length === 0 ? (
        <p class="text-center text-muted">No questions available yet. Submit a prompt!</p>
      ) : (
        <Index each={props.questions}>
          {(q, index) =>
            <QuestionComponent
              question={q()}
              onAnswerChange={(key, newValue) => props.onAnswerChange(index, key, newValue)}
            />
          }
        </Index>
      )}
    </div>
  );
}
