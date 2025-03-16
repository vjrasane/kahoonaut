#[macro_use]
extern crate rocket;
use dotenvy::dotenv;

use mistralai_client::v1::{
    chat::{ChatMessage, ChatMessageRole, ChatParams},
    client::Client,
    constants::Model,
};
use std::io::Cursor;
use tokio::time::{Duration, sleep};

use rocket::{
    Response,
    http::ContentType,
    response::Responder,
    serde::{Deserialize, Serialize, json::Json},
};
use rust_xlsxwriter::Workbook;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
#[derive(Serialize, Deserialize)]
struct Question {
    question: String,
    answers: Vec<String>,
    correct: Vec<u16>,
}

#[post("/api/v1/prompt", format = "text/plain", data = "<prompt>")]
async fn prompt(prompt: String) -> Json<Vec<Question>> {
    let client = Client::new(None, None, None, None).unwrap();

    let model = Model::OpenMistral7b;
    let messages = vec![ChatMessage {
        role: ChatMessageRole::User,
        content: prompt.to_string(),
        tool_calls: None,
    }];
    let options = ChatParams {
        temperature: 0.0,
        random_seed: Some(42),
        ..Default::default()
    };
    sleep(Duration::from_secs(1)).await;
    // let question = result.choices[0].message.content.clone();
    let question = prompt.clone();

    let answers = vec![
        "Placeholder Answer 1".to_string(),
        "Placeholder Answer 2".to_string(),
        "Placeholder Answer 3".to_string(),
        "Placeholder Answer 4".to_string(),
    ];

    let correct = vec![1];

    let response = vec![Question {
        question,
        answers,
        correct,
    }];

    Json(response)
}

struct WorkbooResponse {
    workbook: Workbook,
}

impl<'r> Responder<'r, 'static> for WorkbooResponse {
    fn respond_to(mut self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let mut cursor = Cursor::new(Vec::new());
        self.workbook.save_to_writer(&mut cursor).unwrap();

        debug_assert!(cursor.get_ref().len() > 0);

        Response::build()
            .header(ContentType::Binary)
            .raw_header("Content-Disposition", "attachment; filename=questions.xlsx")
            .sized_body(cursor.get_ref().len(), Cursor::new(cursor.into_inner()))
            .ok()
    }
}

#[post("/export", format = "json", data = "<questions>")]
async fn export_questions(questions: Json<Vec<Question>>) -> WorkbooResponse {
    // Create a new Excel workbook
    let mut workbook = Workbook::new();

    // Add a worksheet
    let worksheet = workbook.add_worksheet();

    // Define the header row
    let headers = [
        "Question",
        "Answer 1",
        "Answer 2",
        "Answer 3",
        "Answer 4",
        "Time limit (sec)",
        "Correct answer(s)",
    ];
    for (col, header) in headers.iter().enumerate() {
        worksheet.write_string(0, col as u16, *header).unwrap();
    }

    // Write question data
    for (row, question) in questions.iter().enumerate() {
        let row = row as u16 + 1; // Offset for the header row

        worksheet
            .write_string(row.into(), 0, &question.question)
            .unwrap();

        // // Write answer choices
        // let mut answer_keys: Vec<_> = question.answers.keys().cloned().collect();
        // answer_keys.sort(); // Ensure order is A, B, C, D

        for (col, answer) in question.answers.iter().enumerate() {
            worksheet
                .write_string(row.into(), (col + 1) as u16, answer)
                .unwrap();
        }

        // Write time limit
        worksheet.write(row.into(), 5, 60).unwrap();
        // Write correct answers
        let correct_answers_str = question
            .correct
            .iter()
            .map(|c| (c + 1).to_string())
            .collect::<Vec<_>>()
            .join(",");
        worksheet
            .write_string(row.into(), 6, &correct_answers_str)
            .unwrap();
    }

    WorkbooResponse { workbook }
}

#[launch]
fn rocket() -> _ {
    dotenv().unwrap();
    rocket::build().mount("/", routes![index, prompt, export_questions])
}
