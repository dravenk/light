// mod generage;
mod nav_items;
mod route;
mod secure;
mod settings;
mod status;

use nav_items::NavItems;
use route::switch;
use route::MainRoute;
use yew::prelude::*;
use yew_router::prelude::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<Main>::new().render();
}

#[function_component(Main)]
pub fn app() -> Html {
    html! {
        <div class="main">
        <BrowserRouter>
            <NavItems />
            <Switch<MainRoute> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
        </div>
    }
}
