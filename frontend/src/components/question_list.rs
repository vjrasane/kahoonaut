use yew::prelude::*;

use super::question::{Question, QuestionComponent};

#[derive(Properties, PartialEq)]
pub struct QuestionsListProps {
    pub questions: Vec<Question>,
    pub on_answer_change: Callback<(usize, String, String)>,
}

#[function_component(QuestionsList)]
pub fn questions_list(props: &QuestionsListProps) -> Html {
    html! {
        <div class="mt-4">

          <h4 class="text-center text-primary">{ "Generated Questions" }</h4>
            <hr />
            {
                if props.questions.is_empty() {
                    html! { <p class="text-center text-muted">{ "No questions yet..." }</p> }
                } else {
                    html! {
                        for props.questions.iter().enumerate().map(|(index, q)| html! {
                            <QuestionComponent question={q.clone()} { index } on_answer_change={props.on_answer_change.clone()} />
                        })
                    }
                }
            }
        </div>
    }
}
