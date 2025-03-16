use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ToastProps {
    pub message: String,
    pub is_success: bool,
    pub on_close: Callback<MouseEvent>,
}

#[function_component(Toast)]
pub fn toast(props: &ToastProps) -> Html {
    let toast_class = if props.is_success {
        "bg-success text-white"
    } else {
        "bg-danger text-white"
    };

    html! {
        <div class={format!("toast show position-fixed bottom-0 end-0 m-3 {}", toast_class)} role="alert">
            <div class="toast-body">
                { &props.message }
                <button type="button" class="btn-close btn-close-white ms-2 me-2" aria-label="Close"
                    onclick={props.on_close.clone()}></button>
            </div>
        </div>
    }
}
