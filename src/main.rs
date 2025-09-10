pub mod pages;
pub mod models;
pub mod state;
pub mod components;

use dioxus::prelude::*;

use crate::pages::{About, Home, Contact, Blog};
use crate::components::navbar::Navbar;

// navbar router setup
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(MainLayout)]
        #[route("/")]
        Home {},
        #[route("/about")]
        About {},
        #[route("/contact")]
        Contact {},
        #[route("/blog/:id")]
        Blog { id: i32 },
}

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css", CssAssetOptions::new().with_preload(true));
const FAV_ICON: Asset = asset!("/assets/favicon.ico");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {use_effect(move || {
    if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
        if let Some(html) = doc.document_element() {
            html.set_attribute("lang", "en").unwrap();
        }
        }
    });
    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS },
        document::Link{ rel: "favicon", href: FAV_ICON},
        document::Meta{name: "description", content:"Club website for T4G @ Pitt."}
        Router::<Route> {}
    }
}

/// Layout applied to all routes (NavBar on top of whatever page is visited)
#[component]
fn MainLayout() -> Element {
    rsx! {
        div {
            class:
            "
                 h-[100dvh] flex flex-col
                 bg-yellow-50
                 bg-[linear-gradient(to_right,#f0f0f0_1px,transparent_1px),linear-gradient(to_bottom,#f0f0f0_1px,transparent_1px)]
                 bg-[size:6rem_4rem]
            ",

            // Navbar is inside Router, so Links work
            Navbar {}

            // Only this area scrolls
            div { class: "flex-1 overflow-y-auto",
                Outlet::<Route> {}
            }
        }
    }
}

// navigation bar used to switch pages / click links
