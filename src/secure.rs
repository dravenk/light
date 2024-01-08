use log::info;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

#[function_component(Secure)]
pub fn secure() -> Html {
    let input_value_handle = use_state(String::default);
    let input_value = (*input_value_handle).clone();

    let on_cautious_change = {
        let input_value_handle = input_value_handle.clone();

        Callback::from(move |e: Event| {
            // When events are created the target is undefined, it's only
            // when dispatched does the target get added.
            let target: Option<EventTarget> = e.target();
            // Events can bubble so this listener might catch events from child
            // elements which are not of type HtmlInputElement
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            println!("input: {:?}", input.clone());
            if let Some(input) = input {
                info!("input_value: {:?}", input.clone().as_string());
                input_value_handle.set(input.value());
            }
        })
    };

    let on_dangerous_change = Callback::from(move |e: Event| {
        let target: EventTarget = e.target().expect("Event should have a target when dispatched");
        // You must KNOW target is a HtmlInputElement, otherwise
        // the call to value would be Undefined Behaviour (UB).
        // Here we are sure that this is input element so we can convert it to the appropriate type without checking
        input_value_handle.set(target.unchecked_into::<HtmlInputElement>().value());
    });

    html! {
        <div class="w-full h-full relative">
            <form class="w-full">
                <div class="absolute bottom-0  w-full mb-4 border border-gray-200 rounded-lg bg-gray-50 dark:bg-gray-700 dark:border-gray-600">
                    <div class="flex items-center justify-between px-3 py-2 border-b dark:border-gray-600">
                    <div class="flex flex-wrap items-center divide-gray-200 sm:divide-x sm:rtl:divide-x-reverse dark:divide-gray-600">
                        <div class="flex items-center space-x-1 rtl:space-x-reverse sm:pe-4">
                            <button type="button" class="p-2 text-gray-500 rounded cursor-pointer hover:text-gray-900 hover:bg-gray-100 dark:text-gray-400 dark:hover:text-white dark:hover:bg-gray-600">
                                <svg class="w-4 h-4" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 12 20">
                                     <path stroke="currentColor" stroke-linejoin="round" stroke-width="2" d="M1 6v8a5 5 0 1 0 10 0V4.5a3.5 3.5 0 1 0-7 0V13a2 2 0 0 0 4 0V6"/>
                                 </svg>
                                <span class="sr-only">{"Attach file"}</span>
                            </button>

                            <button type="button" class="p-2 text-gray-500 rounded cursor-pointer hover:text-gray-900 hover:bg-gray-100 dark:text-gray-400 dark:hover:text-white dark:hover:bg-gray-600">
                                <svg class="w-4 h-4" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 16 20">
                                     <path d="M14.066 0H7v5a2 2 0 0 1-2 2H0v11a1.97 1.97 0 0 0 1.934 2h12.132A1.97 1.97 0 0 0 16 18V2a1.97 1.97 0 0 0-1.934-2ZM10.5 6a1.5 1.5 0 1 1 0 2.999A1.5 1.5 0 0 1 10.5 6Zm2.221 10.515a1 1 0 0 1-.858.485h-8a1 1 0 0 1-.9-1.43L5.6 10.039a.978.978 0 0 1 .936-.57 1 1 0 0 1 .9.632l1.181 2.981.541-1a.945.945 0 0 1 .883-.522 1 1 0 0 1 .879.529l1.832 3.438a1 1 0 0 1-.031.988Z"/>
                                     <path d="M5 5V.13a2.96 2.96 0 0 0-1.293.749L.879 3.707A2.98 2.98 0 0 0 .13 5H5Z"/>
                                 </svg>
                                <span class="sr-only">{"Upload image"}</span>
                            </button>
                        </div>
                    </div>
                </div>
                    <div class="px-4 py-2 bg-white rounded-b-lg dark:bg-gray-800">
                    <label for="editor" class="sr-only">{"Publish post"}</label>
                    <textarea
                    id="editor"
                    rows="8"
                    onchange={on_cautious_change}
                    id="cautious-input"
                    type="text"
                    value={input_value.clone()}
                    class="block w-full px-0 text-sm
                    text-gray-800
                    bg-white 
                    border-0 
                    dark:bg-gray-800
                    focus:ring-0 dark:text-white
                    dark:placeholder-gray-400
                    resize-y
                    " 
                    placeholder="Write an article..."
                    >
                    </textarea>
                </div>
                </div>
            </form>
        // </label>
        <label for="dangerous-input">
            { "My dangerous input:" }
            <input onchange={on_dangerous_change}
                id="dangerous-input"
                type="text"
                value={input_value}
            />
        </label>
    </div>
    }
}
