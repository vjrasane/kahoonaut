import { Question } from "$lib/types";
import type { RequestHandler } from "@sveltejs/kit";
import writeXlsxFile from 'write-excel-file'
import { z } from "zod";

const schema = [
    {
        column: "Question",
        type: String,
        value: (row: Question) => row.question
    },
    {
        column: "Answer 1",
        type: String,
        value: (row: Question) => row.answers[0]
    },
    {
        column: "Answer 2",
        type: String,
        value: (row: Question) => row.answers[1]
    },
    {
        column: "Answer 3",
        type: String,
        value: (row: Question) => row.answers[2]
    },
    {
        column: "Answer 4",
        type: String,
        value: (row: Question) => row.answers[3]
    },
    {
        column: "Time limit (sec)",
        type: Number,
        value: (row: Question) => 60
    },
    {
        column: "Correct answer(s",
        type: String,
        value: (row: Question) => row.correct.join(",")
    }
] as const

export const POST: RequestHandler = async ({ request }) => {
    const questions = z.array(Question).parse(await request.json());
    /* @ts-expect-error */
    const buffer: Buffer = await writeXlsxFile(questions, { schema, buffer: true })
    return new Response(
        buffer,
        {
            headers: {
                "Content-Disposition": "attachment; filename=questions.xlsx",
                "Content-Type": "application/octet-stream"
            }
        }
    )
}