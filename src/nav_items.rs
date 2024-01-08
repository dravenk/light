use yew::prelude::*;
use yew_router::prelude::*;

use crate::MainRoute as main_route;

#[function_component(NavItems)]
pub fn nav_items() -> Html {
    let navigator = use_navigator().unwrap();
    let main_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&main_route::Main));
        html! {
            <button {onclick} class="h-16 px-6 flex flex justify-center items-center w-full
            focus:text-orange-500 bg-inherit">
            <svg
                class="h-5 w-5"
                viewBox="0 0 1024 1024"
                xmlns="http://www.w3.org/2000/svg"
                width="24"
                height="24"
                stroke="black"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                >
             <path d="M1002.7584 475.776L566.2784 39.68 537.1584 10.24a35.584 35.584 0 0 0-50.176 0L21.2544 475.776A72.192 72.192 0 0 0 0.0064 527.744a72.96 72.96 0 0 0 73.28 71.488h48v367.872h781.44V599.232h49.024c19.264 0 37.44-7.552 51.136-21.184 13.696-13.696 21.12-31.872 21.12-51.2 0-19.2-7.552-37.376-21.248-51.072z m-427.52 410.048H448.7744v-230.4h126.464v230.4z m246.144-367.872v367.872h-173.824V628.288a45.184 45.184 0 0 0-45.184-45.184H421.6384a45.184 45.184 0 0 0-45.184 45.184v257.536H202.6304V517.952H94.2144l417.92-417.6 26.048 26.112 391.68 391.488h-108.48z" p-id="12024">
             </path>
             </svg>

        </button>
        }
    };

    let settings_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&main_route::Settings));
        html! {
            <button {onclick} class="h-16 px-6 flex flex justify-center items-center w-full
            focus:text-orange-500 bg-inherit">
            <svg
                class="h-5 w-5"
                xmlns="http://www.w3.org/2000/svg"
                width="24"
                height="24"
                viewBox="0 0 24 24"
                fill="none"
                stroke="black"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round">
                <circle cx="12" cy="12" r="3"></circle>
                <path
                    d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1
                    0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0
                    0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2
                    2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0
                    0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1
                    0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0
                    0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65
                    0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0
                    1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0
                    1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2
                    0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0
                    1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0
                    2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0
                    0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65
                    1.65 0 0 0-1.51 1z"></path>
            </svg>
        </button>
        }
    };
    let status = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&main_route::Status));
        html! {
            <button {onclick}  class="h-16 px-6 flex flex justify-center items-center w-full
            focus:text-orange-500 bg-inherit">
            <svg
            class="h-5 w-5"
            viewBox="0 0 1024 1024"
              xmlns="http://www.w3.org/2000/svg"
              width="24"
              height="24"
                stroke="black"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"><path d="M956.8 128H67.2A67.2 67.2 0 0 0 0 195.2v633.6A67.2 67.2 0 0 0 67.2 896h889.6a67.2 67.2 0 0 0 67.2-67.2V195.2A67.2 67.2 0 0 0 956.8 128zM64 192h896v288h-220.224l-55.168-110.304a31.584 31.584 0 0 0-29.952-17.696 32 32 0 0 0-28.384 20.096l-97.312 243.2-114.976-306.56a32 32 0 0 0-58.592-3.2L268.224 480H64z m896 640H64v-288h224a32 32 0 0 0 28.64-17.696l63.168-126.304 118.24 315.328a32 32 0 0 0 29.632 20.736h0.32a32 32 0 0 0 29.728-20.128l101.472-253.888 32 64.256a32 32 0 0 0 28.8 17.696H960z" fill="" p-id="1930"></path></svg>
            </button>
        }
    };

    let input_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&main_route::Secure));
        html! {
            <button {onclick}  class="h-16 px-6 flex flex justify-center items-center w-full
            focus:text-orange-500 bg-inherit">
            // {"Status"}
                 <svg
                            class="h-5 w-5"
                            xmlns="http://www.w3.org/2000/svg"
                            width="24"
                            height="24"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="black"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round">
                            <path
                                d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9"></path>
                            <path d="M13.73 21a2 2 0 0 1-3.46 0"></path>
                        </svg>
            </button>
        }
    };

    html! {
            <div class="h-full">

            <div class="bg-gray-200 h-full">
              // <!-- container -->
             <aside class="relative bg-white text-gray-700 shadow h-full flex flex-col">
              // <!-- Side Nav Bar-->
              <div class="h-16">
                  {main_button}
              </div>

              <ul class="items-center">
                  <li class="hover:bg-gray-100">
                      {status}
                  </li>
                  <li class="hover:bg-gray-100">
                      {settings_button}
                  </li>
                  <li class="hover:bg-gray-100">
                      {input_button}
                  </li>
              </ul>

              <div class="absolute self-center bottom-10 h-16 w-full">
                  // <!-- Action Section -->
                  <button class="h-16 w-10 mx-auto flex flex justify-center items-center
                      w-full focus:text-orange-500 hover:bg-red-200 focus:outline-none">
                      <svg
                          class="h-5 w-5 text-red-700"
                          xmlns="http://www.w3.org/2000/svg"
                          width="24"
                          height="24"
                          viewBox="0 0 24 24"
                          fill="none"
                          stroke="black"
                          stroke-width="2"
                          stroke-linecap="round"
                          stroke-linejoin="round">
                          <path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"></path>
                          <polyline points="16 17 21 12 16 7"></polyline>
                          <line x1="21" y1="12" x2="9" y2="12"></line>
                      </svg>
                  </button>
              </div>

          </aside>
      </div>
     </div>
    }
}
