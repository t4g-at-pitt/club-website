use dioxus::prelude::*;
use crate::state::use_events_sorted;
use crate::components::event_list::EventList;

#[component]
fn Hero() -> Element {

    rsx! {
        div { class: "p-8 bg-yellow-50 min-h-screen",

            div { class:"flex flex-row justify-around",
                div { class:"flex flex-col max-w-lg prose ",
                    h1 { class:"text-4xl font-bold text-gray-900 mb-4",
                        "Technology for good",
                    }
                    p { class:"text-lg font-medium text-gray-800 mb-4 p-4 bg-white border rounded-xl flex flex-col shadow-sm hover:shadow-md transition-shadow",
                        "Technology for Good (T4G) is dedicated to growing the responsible technology community through learning, career growth, and research enrichment. The organization hosts events centered on hands-on development, understanding the responsible technology landscape, and bettering understanding of research and graduate school opportunities.",
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
    }
}