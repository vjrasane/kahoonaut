export interface Question {
    question: string;
    answers: Record<string, string>;
    correct: string[];
}
