mod components;

use gloo_net::http::Request;
use web_sys::window;
use yew::prelude::*;

use components::prompt_input::PromptInput;

#[function_component(App)]
fn app() -> Html {
    let response = use_state(|| String::new());

    let on_submit = {
        let response = response.clone();
        Callback::from(move |prompt_value: String| {
            let response = response.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match Request::post("/api/v1/prompt")
                    .header("Content-Type", "application/json")
                    .body(format!("{{\"prompt\":\"{}\"}}", prompt_value))
                    .send()
                    .await
                {
                    Ok(resp) => {
                        if let Ok(text) = resp.text().await {
                            response.set(text);
                            window.alert_with_message(&text).ok();
                        }
                    }
                    Err(err) => {
                        response.set(format!("Error: {}", err));
                    }
                }
            });
        })
    };

    html! {
    <div class="container d-flex justify-content-center align-items-center vh-100">
        <div class="card p-4 shadow" style="width: 400px;">
            <PromptInput on_submit={on_submit} />
            <p class="mt-3 text-center text-muted">{ (*response).clone() }</p>
        </div>
    </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
