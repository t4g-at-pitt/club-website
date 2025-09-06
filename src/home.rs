use crate::components::event_list::EventList;
use crate::state::use_events_sorted;
use dioxus::prelude::*;
use crate::components::footer::Footer;

#[component]
fn Hero() -> Element {
    rsx! {
            div { class: "p-8 bg-yellow-50 min-h-screen",

                div { class:"flex flex-row justify-around",
                    div {
        class: "flex flex-col max-w-lg prose-stone",
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

#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}
        Footer{}
    }
}
