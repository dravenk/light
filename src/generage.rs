use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew::{html, Callback, Html, MouseEvent};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

// #[derive(Serialize, Deserialize)]
// struct GreetArgs<'a> {
//     name: &'a str,
// }

#[derive(Serialize, Deserialize)]
struct GenerateOptions<'a> {
    strength: &'a str,
}

#[function_component(Generage)]
pub fn generage() -> Html {
    // wasm_logger::init(wasm_logger::Config::default());

    let generate_input_ref = use_node_ref();

    let strength = use_state(|| String::new());
    let msg = use_state(|| String::new());

    {
        let msg = msg.clone();
        let strength = strength.clone();
        let selected_strength = strength.clone();

        use_effect_with(selected_strength, move |_| {
            spawn_local(async move {
                if strength.is_empty() {
                    return;
                }

                let args = to_value(&GenerateOptions { strength: &*strength }).unwrap();
                let new_msg = invoke("generage_key", args).await.as_string().unwrap();
                msg.set(new_msg);
            });

            || {}
        });
    }

    let generage_key = {
        let strength = strength.clone();
        let generate_input_ref = generate_input_ref.clone();
        let e = move |e: MouseEvent| {
            e.prevent_default();
            strength.set(generate_input_ref.cast::<web_sys::HtmlInputElement>().unwrap().value());
        };
        Callback::from(e)
    };

    html! {
    <main class="container">

      <div class="insert-card">
        <div class="generate-container">
          <div class="sm:flex sm:justify-between sm:space-x-2">
            <select style="display:none;" ref={generate_input_ref} id="strength" class="w-full mb-2 sm:mb-0 strength ok-select sm:w-auto">
              <option value="24" selected=true>{"24 words"}</option>
            </select>
            <button onclick={generage_key} type="button"
              class="inline-flex items-center justify-center w-full px-4 py-2 text-sm font-medium text-white transition bg-green-600 border border-transparent rounded-md shadow-sm generate hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-green-500 sm:w-auto">
              <svg style="width:10px" xmlns="http://www.w3.org/2000/svg" class="w-5 h-5 mr-2 -ml-1"
                viewBox="0 0 20 20" fill="currentColor">
                <path fill-rule="evenodd"
                  d="M4 2a1 1 0 011 1v2.101a7.002 7.002 0 0111.601 2.566 1 1 0 11-1.885.666A5.002 5.002 0 005.999 7H9a1 1 0 010 2H4a1 1 0 01-1-1V3a1 1 0 011-1zm.008 9.057a1 1 0 011.276.61A5.002 5.002 0 0014.001 13H11a1 1 0 110-2h5a1 1 0 011 1v5a1 1 0 11-2 0v-2.101a7.002 7.002 0 01-11.601-2.566 1 1 0 01.61-1.276z"
                  clip-rule="evenodd"></path>
              </svg>
              {"GENERATE"}
            </button>
          </div>
        </div>
        <div>
        </div>
        <p><b>{ &*msg }</b></p>

      </div>
    </main>
    }
}
