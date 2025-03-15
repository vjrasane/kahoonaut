mod components;
use std::collections::HashMap;

use gloo_net::Error;
use gloo_net::http::{Request, Response};
use serde::Deserialize;
use web_sys::window;
use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use components::prompt_input::PromptInput;
#[derive(Deserialize, Clone, PartialEq)]
struct QuizResponse {
    question: String,
    answers: HashMap<String, String>,
    correct_answer: String,
}

#[function_component(App)]
fn app() -> Html {
    let response = use_state(|| String::new());

    let on_submit = {
        let response = response.clone();
        Callback::from(move |prompt_value: String| {
            let response = response.clone();
            spawn_local(async move {
                let resp = Request::post("/api/v1/prompt")
                    .header("Content-Type", "text/plain")
                    .body(prompt_value)
                    .send()
                    .await;
                let text = handle_response(resp).await;
                response.set(text);
            });
        })
    };

    html! {
    <div class="container d-flex justify-content-center align-items-center vh-100">
        <div class="card p-4 shadow" style="width: 400px;">
            <PromptInput on_submit={on_submit} />
            <p class="mt-3 text-center text-muted">{ ( *response ).clone() }</p>
        </div>
    </div>
    }
}

async fn handle_response(resp: Result<Response, Error>) -> String {
    match resp {
        Ok(resp) => {
            let question = resp.json::<QuizResponse>().await.unwrap();
            let status = resp.status();
            let text = format!("Status: {}\n{}", status, question.question);
            window().unwrap().alert_with_message(&text).ok();
            text
        }
        Err(err) => {
            let error_message = format!("Error: {}", err);
            window().unwrap().alert_with_message(&error_message).ok();
            error_message
        }
    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}
