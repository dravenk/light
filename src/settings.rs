use log;
use wasm_bindgen::prelude::*;
use yew::html;
use yew::prelude::*;

use serde::{Deserialize, Serialize};

use serde_wasm_bindgen::to_value;
use wasm_bindgen_futures::spawn_local;

use yew::{Callback, Html};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GenerateOptions<'a> {
    strength: &'a str,
}

#[function_component(Settings)]
pub fn settings() -> Html {
    let input_node_ref = use_node_ref();
    let words = use_state(|| vec![String::new(); 24]);
    let strength = use_state(|| String::new());
    {
        // let words = words.clone();
        let strength = strength.clone();
        let selected_strength = strength.clone();

        use_effect_with(selected_strength, move |_| {
            log::info!("I'm a effect!");
            spawn_local(async move {});
            || {}
        });
    }

    let callback = {
        log::info!("I'm a onchange!");
        let strength_clone = strength.clone();
        let words_clone = words.clone();
        let e = move |e: MouseEvent| {
            e.prevent_default();
            let strength = strength_clone.clone();
            let words = words_clone.clone();

            spawn_local(async move {
                let args = to_value(&GenerateOptions { strength: &*strength }).unwrap();
                let new_msg = invoke("generage_key", args).await.as_string().unwrap();
                let w = new_msg.split(" ").collect::<Vec<&str>>();
                words.set(w.iter().map(|s| s.to_string()).collect::<Vec<String>>());
            });
        };

        Callback::from(e)
    };

    html! {
         <div class="flex wrap column" style="">
          <div class="row mw-75" >
          {
             for (1..25).map(|i| {
            let words = words.clone();
            let word = words[i-1].clone();
            let word_clone = word.to_owned().clone();
            let input_id = "input-".to_string() + i.to_string().as_str();
            let input_node_ref = input_node_ref.clone();

            html! {
                <div class="flex items-center mx-2 my-0 px-2 py-1 h-14 w-40" >
                    <div class="number mr-1.5">{ i.clone() }</div>
                    <input
                    class="w-full "
                    disabled={true}
                    placeholder={ word_clone.clone()}
                    id={ input_id }
                        type="text"
                        value={word_clone.clone()}
                        ref={input_node_ref}
                    />
                </div>
            }
        }) }
          </div>
          <div class="flex">
          <button class="mx-4" onclick={ callback }>
          { "Click me!" }
            </button>
         </div>
         </div>
    }
}
