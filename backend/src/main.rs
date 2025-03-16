#[macro_use]
extern crate rocket;
use rocket::serde::{ Serialize, json::Json};
use std::collections::HashMap;
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
#[derive(Serialize)]
struct Question {
    question: String,
    answers: HashMap<String, String>,
    correct: Vec<String>,
}

#[post("/api/v1/prompt", format = "text/plain", data = "<prompt>")]
fn prompt(prompt: String) -> Json<Vec<Question>> {
    let question = prompt.clone();
    
    let mut answers = HashMap::new();
    answers.insert("A".to_string(), "Placeholder Answer 1".to_string());
    answers.insert("B".to_string(), "Placeholder Answer 2".to_string());
    answers.insert("C".to_string(), "Placeholder Answer 3".to_string());
    answers.insert("D".to_string(), "Placeholder Answer 4".to_string());

    let correct = vec!["A".to_string()]; 

    let response = vec![Question {
        question,
        answers,
        correct,
    }];

    Json(response)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, prompt])
}
