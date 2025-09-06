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

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const FAV_ICON: Asset = asset!("/assets/favicon.ico");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS },
        document::Link{ rel: "favicon", href: FAV_ICON},
        Router::<Route> {}
    }
}

/// Layout applied to all routes (NavBar on top of whatever page is visited)
#[component]
fn MainLayout() -> Element {
    rsx! {
        div { class: "h-screen flex flex-col",

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
