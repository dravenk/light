use log::info;
use wasm_bindgen::JsCast;
use web_sys::KeyboardEvent;
use web_sys::{EventTarget, HtmlTextAreaElement};
use yew::prelude::*;

use super::super::ipfs;

#[function_component(Chat)]
pub fn chat() -> Html {
    let show_msg_handle: UseStateHandle<String> = use_state(String::default);
    let show_msg = (*show_msg_handle).clone();
    let input_value_handle = use_state(String::default);
    let input_value = (*input_value_handle).clone();

    let on_cautious_change = {
        let input_value_handle = input_value_handle.clone();

        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok());
            info!("input_value: {:?}", input.clone());
            println!("input: {:?}", input.clone());
            if let Some(input) = input {
                info!("input_value: {:?}", input.clone().as_string());
                input_value_handle.set(input.value());
            }
        })
    };

    let onkeypress = {
        let input_value_handle = input_value_handle.clone();
        let show_msg_handle = show_msg_handle.clone();

        Callback::from(move |e: KeyboardEvent| {
            if !e.shift_key() {
                // Enter| e.code(): "Enter" | e.key(): "Enter" |  e.key_code() == 13
                let target: Option<EventTarget> = e.target();
                let input = target.and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok());
                if e.key() == "Enter" {
                    if let Some(input) = input {
                        info!("input_value: {:?}", input.clone().as_string());
                        let cid = ipfs::cid::bytes_to_cid(input.value().as_bytes());
                        let msg = cid.unwrap().to_string();
                        // All data should be encrypted before sending to ipfs
                        show_msg_handle.set(msg);
                        input_value_handle.set(String::default());
                    }
                }
            } else {
                info!("shift_key: {:?}", e.shift_key());
            }
        })
    };
    html! {
        <div class="w-full h-full relative">
            <form class="w-full  absolute bottom-4 ">
                <div class="w-full mb-4 border border-gray-200 rounded-lg bg-gray-50">
                    <div class="flex items-center justify-between px-3 py-2 border-b">
                        <div class="flex flex-wrap items-center divide-gray-200 sm:divide-x sm:rtl:divide-x-reverse">
                            <div class="flex items-center space-x-1 rtl:space-x-reverse sm:pe-4">
                                <button type="button" class="p-2 text-gray-500 rounded cursor-pointer hover:text-gray-900 hover:bg-gray-100">
                                    <svg class="w-4 h-4" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 12 20">
                                         <path stroke="currentColor" stroke-linejoin="round" stroke-width="2" d="M1 6v8a5 5 0 1 0 10 0V4.5a3.5 3.5 0 1 0-7 0V13a2 2 0 0 0 4 0V6"/>
                                     </svg>
                                    <span class="sr-only">{"Attach file"}</span>
                                </button>

                                <button type="button" class="p-2 text-gray-500 rounded cursor-pointer hover:text-gray-900 hover:bg-gray-100">
                                    <svg class="w-4 h-4" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 16 20">
                                         <path d="M14.066 0H7v5a2 2 0 0 1-2 2H0v11a1.97 1.97 0 0 0 1.934 2h12.132A1.97 1.97 0 0 0 16 18V2a1.97 1.97 0 0 0-1.934-2ZM10.5 6a1.5 1.5 0 1 1 0 2.999A1.5 1.5 0 0 1 10.5 6Zm2.221 10.515a1 1 0 0 1-.858.485h-8a1 1 0 0 1-.9-1.43L5.6 10.039a.978.978 0 0 1 .936-.57 1 1 0 0 1 .9.632l1.181 2.981.541-1a.945.945 0 0 1 .883-.522 1 1 0 0 1 .879.529l1.832 3.438a1 1 0 0 1-.031.988Z"/>
                                         <path d="M5 5V.13a2.96 2.96 0 0 0-1.293.749L.879 3.707A2.98 2.98 0 0 0 .13 5H5Z"/>
                                     </svg>
                                    <span class="sr-only">{"Upload image"}</span>
                                </button>
                            </div>
                        </div>
                    </div>
                    <div class="px-4 py-2 bg-white rounded-b-lg">
                        <label for="editor" class="sr-only">{"Publish post"}</label>

                        <textarea
                        rows="8"
                        placeholder="Write an article..."
                        onchange={on_cautious_change}
                        onkeypress={onkeypress}
                        id="cautious-input"
                        type="text"
                        value={input_value.clone()}
                        class="block w-full px-0 text-sm text-gray-800
                        bg-white  border-0 focus:ring-0"
                        >
                        </textarea>

                    </div>
                </div>
            </form>
        <div class="show-message">{show_msg}</div>
    </div>
    }
}
