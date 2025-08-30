pub mod home;
pub mod blog;

use home::Home;
use blog::Blog;
use dioxus::prelude::*;

// navbar router setup
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(MainLayout)]
        #[route("/")]
        Home {},
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
#[component]
fn Navbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            class:"sticky top-0 z-10 p-2 flex flex-row bg-blue-200 border",

            // Main logo / title of navbar
            p { class:"text-2xl mr-4 ml-2 font-semibold",
                "Technology for Good @ Pitt"
            }

            // list of links
            div { class:"ml-auto space-x-4",
                Link { class:"text-xl", to: Route::Home {}, "Home" },
                Link { class:"text-xl", to: "https://github.com/t4g-at-pitt", new_tab: true, "Projects" },
                Link { class:"text-xl", to: Route::Home {}, "Contact" },
                Link { class:"text-xl", to: Route::Home {}, "About" },
                // Link { class:"text-xl", to: Route::Blog { id: 1 }, "Blog" }
            }
        }
    }
}
