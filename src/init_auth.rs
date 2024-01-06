use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
pub struct GithubArgs<'a> {
    name: &'a str,
}

#[function_component(InitAuth)]
pub fn init_auth() -> Html {
    let auth_ref = use_node_ref();

    let name = use_state(|| String::new());

    let msg = use_state(|| String::new());
    {
        let msg = msg.clone();
        let name = name.clone();
        let name2 = name.clone();
        use_effect_with(name2, move |_| {
            spawn_local(async move {
                if name.is_empty() {
                    return;
                }
                let args = to_value(&GithubArgs { name: &*name }).unwrap();
                let new_msg = invoke("insert_msg", args).await.as_string().unwrap();
                msg.set(new_msg);
            });

            || {}
        });
    }

    let greet = {
        let name = name.clone();
        let auth_ref = auth_ref.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            name.set(
                auth_ref
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value(),
            );
        })
    };

    html! {
        <main class="container">
            // <div class="row">
            // </div>
            <form class="row" onsubmit={greet}>
                <input id="auth_ref" ref={auth_ref} placeholder="Enter message..." />
                <button type="submit">{"submit"}</button>
            </form>

            <p><b>{ &*msg }</b></p>
        </main>
    }
}
