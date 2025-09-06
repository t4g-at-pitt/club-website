use crate::Route;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
#[component]
pub fn Navbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            class:"sticky top-0 z-10 p-2 flex flex-row bg-blue-100 border items-center justify-center",

            // Main logo / title of navbar
            h1 { class:"mr-4 ml-2 text-3xl font-semibold text-black",
                "T4G @ Pitt"
            }

            // list of links
            div { class:"ml-auto mr-2 space-x-4 items-center justify-center",
                Link { class:"text-xl text-blue-950 hover:underline", to: Route::Home {}, "Home" },
                Link { class:"text-xl text-blue-950 hover:underline", to: "https://github.com/t4g-at-pitt", new_tab: true, "Projects" },
                Link { class:"text-xl text-blue-950 hover:underline", to: Route::Contact {}, "Contact" },
                Link { class:"text-xl text-blue-950 hover:underline", to: Route::About {}, "About" },
                // Link { class:"text-xl", to: Route::Blog { id: 1 }, "Blog" }
            }
        }
    }
}