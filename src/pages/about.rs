use crate::state::use_info_cards;
use dioxus::core_macro::{component, rsx};
use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    let info_cards = use_info_cards();
    use dioxus_motion::prelude::*;
    let mut opacity = use_motion(0.0f32);
    use_effect(move || {
        opacity.animate_to(
            1.0, // target opacity
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 100.0,
                damping: 10.0,
                mass: 0.5,
                velocity: 0.0,
            }))
        );
    });

    rsx! {
    div {
        class: "flex flex-col items-center p-10 min-h-[100dvh] bg-yellow-50 bg-[linear-gradient(to_right,#f0f0f0_1px,transparent_1px),linear-gradient(to_bottom,#f0f0f0_1px,transparent_1px)] bg-[size:6rem_4rem]",
        style: "opacity: {opacity.get_value()}; transition: opacity 0.1s;",

        // Container for both header and cards
        div {
            class: "w-full max-w-5xl flex flex-col",

            // Header aligned with leftmost card
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

            // Cards centered
            main {
                div {
                    class: "grid grid-cols-1 md:grid-cols-2 gap-6 justify-center", // centers the grid on page
                    for card in info_cards.read().iter() {
                        div {
                            key: "{card.title()}",
                            class: "prose p-4 md:max-w-xl bg-white border rounded-xl shadow-sm hover:shadow-md transition-shadow",
                            h2 { "{card.title()}" }
                            p { "{card.description()}" }
                        }
                    }
                }
            }
        }
    }
}

}
