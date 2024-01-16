use yew::prelude::*;
use yew_router::prelude::*;

mod conversation;
mod ipfs;
mod nav;
mod route;
mod settings;
mod status;
use nav::Nav;
use route::switch;
use route::MainRoute;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<Main>::new().render();
}

#[function_component(Main)]
pub fn app() -> Html {
    html! {
        <div class="main overflow-hidden h-screen w-screen">

        // <!-- page -->
        <BrowserRouter>
        <main class="h-full w-full bg-gray-100 text-gray-700">
            // <!-- header page -->
            <header class="flex w-full items-center justify-between border-b-2 border-gray-200 bg-white p-2">
                // <!-- logo -->
                <div class="flex items-center space-x-2">
                    <div>{"Me"}</div>
                </div>
            </header>

            <div class="flex w-full h-full">
                <div class="flex h-screen">
                 <Nav />
                </div>
                // <!-- main content page -->
                <div class="w-full p-4">
                <Switch<MainRoute> render={switch} /> // <- must be child of <BrowserRouter>
               </div>
            </div>
        </main>
        </BrowserRouter>

        </div>

    }
}
