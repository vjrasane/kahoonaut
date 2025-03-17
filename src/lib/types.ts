import { z } from 'zod';

export const Question = z.object({
    question: z.string(),
    answers: z.array(z.string()),
    correct: z.array(z.number())
})

export type Question = z.infer<typeof Question>

export const PromptInput = z.object({
    prompt: z.string(),
    excludeQuestions: z.array(Question),
    language: z.enum(['finnish', 'english'])
})

export type PromptInput = z.infer<typeof PromptInput>