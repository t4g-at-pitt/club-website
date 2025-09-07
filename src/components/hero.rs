use crate::components::event_list::EventList;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {

    // Fade-in animation
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
        // outer hero container
        div {
            class: "overflow-x-hidden min-h-[80vh] flex flex-col justify-center
                    bg-yellow-50
                    bg-[linear-gradient(to_right,#f0f0f0_1px,transparent_1px),linear-gradient(to_bottom,#f0f0f0_1px,transparent_1px)]
                    bg-[size:6rem_4rem]",
            style: "opacity: {opacity.get_value()}; transition: opacity 0.2s;",

            div { class: "p-10 lg:p-16 flex flex-col md:flex-row justify-center items-center gap-y-8 md:gap-x-12 w-full",

                // Left text column
                div {
                    class: "flex flex-col max-w-sm lg:max-w-xl prose-stone",
                    h1 {
                        class: "text-4xl font-bold text-gray-900 mb-2",
                        "Technology for Good"
                    }
                    h1 {
                        class: "text-2xl font-medium text-gray-700 mb-3",
                        "at the University of Pittsburgh"
                    }
                    p {
                        class: "text-lg lg:text-xl text-gray-800 mb-4 p-6 bg-white border
                                rounded-xl shadow-sm hover:shadow-md transition-shadow",
                        "The organization ",
                        span { class: "font-bold text-gray-900", "Technology for Good (T4G)" },
                        " is dedicated to growing the ",
                        span { class: "font-semibold text-indigo-600", "responsible technology community" },
                        " through learning, ",
                        span { class: "font-semibold", "career growth" },
                        ", and ",
                        span { class: "font-semibold", "research enrichment" },
                        ". The organization hosts events centered on ",
                        span { class: "italic", "hands-on development" },
                        ", understanding the ",
                        span { class: "underline text-indigo-500", "responsible technology landscape" },
                        ", and better understanding of research and graduate school opportunities. 🚀💡"
                    }
                }

                // Right event list
                div { class:"w-full max-w-3xl",
                    EventList {}
                }
            }
        }
    }
}
