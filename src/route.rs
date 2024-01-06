// use crate::generage::Generage;
use crate::secure::Secure;
use crate::settings::Settings;
use crate::status::Status;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum MainRoute {
    #[at("/")]
    Main,
    #[at("/secure")]
    Secure,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/settings")]
    Settings,
    #[at("/status")]
    Status,
    // #[at("/key")]
    // Generage,
}

pub fn switch(route: MainRoute) -> Html {
    match route {
        MainRoute::Main => html! { <h1>{ "Main" }</h1> },
        MainRoute::Secure => html! {
            <Secure />
        },
        MainRoute::NotFound => html! { <h1>{ "404" }</h1> },
        MainRoute::Settings => {
            html! {
                <Settings />
            }
        }
        MainRoute::Status => {
            html! {
                <Status />
            }
        }
        // MainRoute::Misc { path } => html! {<p>{format!("Matched some other path: {}", path)}</p>},
        // MainRoute::Generage => html! {
        //     <Generage />
        // },
    }
}
