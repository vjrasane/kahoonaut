mod components;

use components::prompt_input::PromptInput;
use components::question::Question;
use components::question_list::QuestionsList;
use components::toast::Toast;
use gloo_net::Error;
use gloo_net::http::{Request, Response};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let questions = use_state(|| vec![]);
    let show_toast = use_state(|| None::<(String, bool)>);

    let on_answer_change = {
        let questions = questions.clone();
        Callback::from(
            move |(index, key, new_value): (usize, String, String)| {
                let mut updated_questions = (*questions).clone();
                if let Some(q) = updated_questions.get_mut(index) {
                    if let Some(answer) = q.answers.get_mut(&key) {
                        *answer = new_value;
                    }
                }
                questions.set(updated_questions);
            },
        )
    };

    let on_submit = {
        let questions = questions.clone();
        let show_toast = show_toast.clone();
        Callback::from(move |prompt_value: String| {
            let questions = questions.clone();
            let show_toast = show_toast.clone();
            spawn_local(async move {
                let resp = Request::post("/api/v1/prompt")
                    .header("Content-Type", "text/plain")
                    .body(prompt_value)
                    .send()
                    .await;
                let result = handle_response(resp).await;
                match result {
                    Ok(qs) => {
                        questions.set(qs);
                    }
                    Err(err) => {
                        show_toast.set(Some((err, false)));
                    }
                }
            });
        })
    };

    let close_toast = {
        let show_toast = show_toast.clone();
        Callback::from(move |_| show_toast.set(None))
    };

    html! {
    <div class="container d-flex justify-content-center align-items-center vh-100">
        <div class="card p-4 shadow" style="width: 400px;">
            <PromptInput on_submit={on_submit} />
            <QuestionsList questions={( *questions ) .clone()} />
             { if let Some((message, is_success)) = (*show_toast).clone() {
                 html! { <Toast message={message} is_success={is_success} on_close={close_toast.clone()} /> }
             } else {
                 html! {}
             }}
        </div>
    </div>
    }
}

async fn handle_response(resp: Result<Response, Error>) -> Result<Vec<Question>, String> {
    match resp {
        Ok(resp) => {
            let questions = resp.json::<Vec<Question>>().await.unwrap();
            Ok(questions)
        }
        Err(err) => {
            let error_message = format!("Error: {}", err);
            Err(error_message)
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
