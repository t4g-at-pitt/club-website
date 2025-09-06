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
            class:"sticky top-0 z-10 p-2 flex flex-row bg-blue-100 border items-center justify-center",

            // Main logo / title of navbar
            h1 { class:"mr-4 ml-2 text-3xl font-semibold text-black",
                "T4G @ Pitt"
            }

            // desktop links
            div { class:"hidden md:flex ml-auto mr-2 space-x-4 items-center justify-center",
                Link { class:"text-xl text-blue-950 hover:underline", to: Route::Home {}, "Home" },
                Link { class:"text-xl text-blue-950 hover:underline", to: "https://github.com/t4g-at-pitt", new_tab: true, "Projects" },
                Link { class:"text-xl text-blue-950 hover:underline", to: Route::Contact {}, "Contact" },
                Link { class:"text-xl text-blue-950 hover:underline", to: Route::About {}, "About" },
            }

            // mobile menu button
            button {
                class: "md:hidden ml-auto mr-4",
                onclick: toggle_menu,
                img {
                    class: "size-6",
                    src: asset!("assets/icons/menu-button.svg"),
                    alt: "Menu"
                }
            }
        }

        // mobile dropdown menu
        // mobile dropdown menu
        if menu_open() {
            div {
                class: "md:hidden flex flex-col bg-blue-50 w-full p-2 border space-y-4 absolute top-12 z-50",
                Link { class:"text-xl text-blue-950 text-center", to: Route::Home {}, "Home" },
                Link { class:"text-xl text-blue-950 text-center", to: "https://github.com/t4g-at-pitt", new_tab: true, "Projects" },
                Link { class:"text-xl text-blue-950 text-center", to: Route::Contact {}, "Contact" },
                Link { class:"text-xl text-blue-950 text-center", to: Route::About {}, "About" },
            }
        }
    }
}
