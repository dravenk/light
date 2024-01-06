use yew::prelude::*;
use yew_router::prelude::*;

use crate::MainRoute as main_route;

#[function_component(NavItems)]
pub fn nav_items() -> Html {
    let navigator = use_navigator().unwrap();

    // let generage_key = {
    //     let navigator = navigator.clone();
    //     let onclick = Callback::from(move |_| navigator.push(&main_route::Generage));
    //     html! {
    //         <button {onclick}>{"Key"}</button>
    //     }
    // };

    let go_main_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&main_route::Main));
        html! {
            <button {onclick}>{"Main"}</button>
        }
    };

    let settings_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&main_route::Settings));
        html! {
            <button {onclick}>
            {"Settings"}
            </button>
        }
    };
    let status = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&main_route::Status));
        html! {
            <button {onclick}>
            {"Status"}
            </button>
        }
    };
    let go_to_secure_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&main_route::Secure));
        html! {
            <button {onclick}>{"Secure"}</button>
        }
    };

    html! {
        <div class="column d-flex" style="margin-left: 10px; margin-right: 10px;">
            {go_main_button}
            // {generage_key}
            {settings_button}
            {status}
            {go_to_secure_button}
        </div>
    }
}
