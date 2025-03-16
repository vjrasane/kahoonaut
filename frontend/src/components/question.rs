use serde::Deserialize;
use std::collections::HashMap;
use yew::prelude::*;

#[derive(Deserialize, Clone, PartialEq)]
pub struct Question {
    pub question: String,
    pub answers: HashMap<String, String>,
    pub correct_answer: String,
}

#[derive(Properties, PartialEq)]
pub struct QuestionProps {
    pub question: Question,
    pub index: usize,
    pub on_answer_change: Callback<(usize, String, String)>,
}

#[function_component(QuestionComponent)]
pub fn question_component(props: &QuestionProps) -> Html {
    let on_input = {
        let index = props.index;
        let on_answer_change = props.on_answer_change.clone();
        Callback::from(move |e: InputEvent| {
            let target = e.target_dyn_into::<web_sys::HtmlInputElement>();
            if let Some(input) = target {
                let key = input.get_attribute("data-key").unwrap_or_default();
                let value = input.value();
                on_answer_change.emit((index, key, value));
            }
        })
    };

    html! {
    <div class="card mb-3 p-3 shadow-sm">
                <h5>{ &props.question.question }</h5>
                <div class="row g-2 mt-2">
                    { for props.question.answers.iter().enumerate().map(|(index, (key, value))| html! {
                     <div class="col-6">
                        <input type="text" class="form-control" value={value.clone()} data-key={key.clone()}
                            oninput={on_input.clone()} />
                    </div>
                    }) }
                </div>
                <p class="mt-3"><strong>{ "Correct Answer: " }</strong>{ &props.question.correct_answer }</p>
            </div>
        }
}
