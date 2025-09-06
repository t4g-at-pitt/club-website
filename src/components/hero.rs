use crate::components::event_list::EventList;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
    rsx! {
        div { class:"z-[-2] bg-yellow-50 bg-[linear-gradient(to_right,#f0f0f0_1px,transparent_1px),linear-gradient(to_bottom,#f0f0f0_1px,transparent_1px)] bg-[size:6rem_4rem]",

        div { class: "p-10",

            div { class:"flex flex-col md:flex-row justify-around gap-y-8 md:gap-x-8",
                div {
                    class: "flex flex-col max-w-sm lg:max-w-lg prose-stone",
                    h1 {
                        class: "text-4xl font-bold text-gray-900 mb-4",
                        "Technology for Good",
                    }
                    p {
                        class: "text-lg text-gray-800 mb-4 p-6 bg-white border rounded-xl shadow-sm hover:shadow-md transition-shadow",
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
                div { class:"w-xl max-w-3xl",
                        EventList {}
                    }
                }
            }
        }
    }
}
