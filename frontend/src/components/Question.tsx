
export type Question = {
  question: string;
  answers: Record<string, string>;
  correctAnswer: string;
}

export function QuestionComponent(props: {
  question: Question;
  onAnswerChange: (key: string, newValue: string) => void;
}) {

  const handleInputChange = (key: string, event: Event) => {
    const value = (event.target as HTMLInputElement).value;
    props.onAnswerChange(key, value);
  };

  return (
    <div class="card mb-3 p-3 shadow-sm" style={{ "max-width": "600px" }}>
      <h5>{props.question.question}</h5>
      <div class="row g-2 mt-2">
        {Object.entries(props.question.answers).map(([key, value]) => (
          <div class="col-6">
            <input
              type="text"
              class="form-control"
              value={value}
              data-key={key}
              onInput={(e) => handleInputChange(key, e)}
            />
          </div>
        ))}
      </div>
      <p class="mt-3">
        <strong>Correct Answer: </strong>
        {props.question.correctAnswer}
      </p>
    </div>
  );
}
