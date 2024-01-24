use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::html;
use yew::prelude::*;
use yew::Html;

// https://github.com/tauri-apps/tauri/issues/8073#issuecomment-1773530639
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[function_component(Status)]
pub fn status() -> Html {
    // PEER ID
    let peer_id_handle: UseStateHandle<String> = use_state(String::default);
    let peer_id = (*peer_id_handle).clone();
    // let address = use_state(|| vec![String::new()]);
    {
        let pid_handle = peer_id_handle.clone();
        use_effect_with(pid_handle, move |_| {
            spawn_local(async move {
                let s = String::from("{}");
                let args = serde_wasm_bindgen::to_value(&s).unwrap();
                let peer_id = invoke("get_peer_id", args).await.as_string().unwrap();
                peer_id_handle.set(peer_id);
            });
            || {}
        });
    }

    html! {
        <div>

                <h1>{ "Status" }</h1>

                <div class="">

                <div class="">{"PEER ID "}<div class="">{peer_id} </div></div>
                </div>
        </div>
    }
}
