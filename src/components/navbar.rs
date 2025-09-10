use crate::Route;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    // Signal to track mobile menu open/close state
    let mut menu_open = use_signal(|| false);

    let toggle_menu = move |_| {
        menu_open.set(!menu_open());
    };

    rsx! {
        div {
            id: "navbar",
            class:"sticky top-0 z-10 p-2 flex flex-row border-b bg-blue-100 items-center justify-center border-blue-200 border-b-5 rounded-b-xl",

            // Main logo / title of navbar
            h1 { class:"mr-4 ml-2 text-3xl font-semibold text-black",
                "T4G @ Pitt"
            }

            // desktop links
            div { class:"hidden md:flex ml-auto mr-2 space-x-4 items-center justify-center",
Link { class:"text-xl font-medium text-blue-950 hover:text-sky-600 transition-all duration-200", to: Route::Home {}, "Home" },
                Link { class:"text-xl font-medium text-blue-950 hover:text-sky-600 transition-all duration-200", to: Route::About {}, "About" },
                Link { class:"text-xl font-medium text-blue-950 hover:text-sky-600 transition-all duration-200", to: Route::Contact {}, "Contact" },
                Link { class:"text-xl font-medium text-blue-950 hover:text-sky-600 transition-all duration-200", to: "https://github.com/t4g-at-pitt", new_tab: true, "Projects" },
            }

            // mobile menu button
            button {
                class: "md:hidden ml-auto mr-4",
                onclick: toggle_menu,
                img {
                    class: "size-6",
                    style: "width: 1.5rem;height: 1.5rem;",
                    src: asset!("assets/icons/menu-button.svg", ImageAssetOptions::new().with_avif()),
                    alt: "Menu"
                }
            }
        }

        // mobile dropdown menu
        // mobile dropdown menu
       // mobile dropdown menu
        if menu_open() {
            div {
                class: "items-center flex flex-col justify-center mx-auto space-x-2 md:hidden bg-blue-50 p-2 border w-full space-y-2 absolute top-13 z-50",

                Link {
                    class:"text-xl text-blue-950 text-center bg-blue-300 border-b-4 border-blue-500 w-fit px-2 rounded-xl",
                    to: Route::Home {},
                    onclick: move |_| menu_open.set(false),
                    "Home"
                },

                Link {
                    class:"text-xl text-blue-950 text-center bg-red-300 border-b-4 border-red-500 w-fit px-2 rounded-xl",
                    to: Route::About {},
                    onclick: move |_| menu_open.set(false),
                    "About"
                },

                Link {
                    class:"text-xl text-blue-950 text-center bg-yellow-300 border-b-4 border-yellow-500 w-fit px-2 rounded-xl",
                    to: Route::Contact {},
                    onclick: move |_| menu_open.set(false),
                    "Contact"
                },

                Link {
                    class:"text-xl text-blue-950 text-center bg-green-300 border-b-4 border-green-500 w-fit px-2 rounded-xl",
                    to: "https://github.com/t4g-at-pitt",
                    new_tab: true,
                    onclick: move |_| menu_open.set(false),
                    "Projects"
                },
            }
        }
    }
}
