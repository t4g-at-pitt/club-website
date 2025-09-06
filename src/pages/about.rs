use crate::state::use_info_cards;
use dioxus::core_macro::{component, rsx};
use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    let info_cards = use_info_cards();

    rsx! {
        div { class: "flex flex-col p-10 bg-yellow-50 h-full",
            header {
                class: "mb-6",
                h1 {
                    class: "text-4xl font-bold text-gray-900 mb-2",
                    "About Our Club"
                }
                p {
                    class: "text-gray-700 text-lg",
                    "Learn more about what we do, how to get involved, and our ongoing projects."
                }
            }

            main {
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                    for card in info_cards.read().iter() {
                        div {
                            key: "{card.title()}",
                            class: "prose p-4 max-w-xs md:max-w-xl bg-white border rounded-xl shadow-sm hover:shadow-md transition-shadow",
                            h2 { "{card.title()}" }
                            p { "{card.description()}" }
                        }
                    }
                }
            }
        }
    }
}
