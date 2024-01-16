use yew::prelude::*;
use yew_router::prelude::*;

use crate::conversation::Conversation;
use crate::settings::Settings;
use crate::status::Status;

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum MainRoute {
    #[at("/")]
    Main,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/status")]
    Status,
    #[at("/settings")]
    Settings,
    #[at("/conversation")]
    Conversation,
}

pub fn switch(route: MainRoute) -> Html {
    match route {
        MainRoute::Main => html! { <h1>{ "Main" }</h1> },
        MainRoute::NotFound => html! { <h1>{ "404" }</h1> },
        MainRoute::Status => {
            html! {
                <Status />
            }
        }
        MainRoute::Settings => {
            html! {
                <Settings />
            }
        }
        MainRoute::Conversation => html! {
            <Conversation />
        },
    }
}
