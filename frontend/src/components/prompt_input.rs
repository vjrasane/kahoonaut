use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlTextAreaElement, InputEvent};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub on_submit: Callback<String>,
}

#[function_component(PromptInput)]
pub fn prompt_input(props: &Props) -> Html {
    let prompt = use_state(|| String::new());

    let on_input = {
        let prompt = prompt.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlTextAreaElement = e.target().unwrap().dyn_into::<HtmlTextAreaElement>().unwrap();
            prompt.set(input.value());
        })
    };

    let on_submit = {
        let prompt = prompt.clone();
        let on_submit = props.on_submit.clone();
        Callback::from(move |_| {
            on_submit.emit((*prompt).clone());
        })
    };

    html! {
        <>
            <textarea class="form-control mb-3" oninput={on_input} rows=5 placeholder="Enter your prompt here..."/>
            <button class="btn btn-primary w-100" onclick={on_submit}>{ "Submit" }</button>
        </>
    }
}
