use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            class: "relative h-screen w-screen bg-yellow-50 overflow-y-auto",

            // Linear grid background - correct Tailwind syntax
            div {
                class: "absolute inset-0 z-0",
                style: "background-image:
                        linear-gradient(to right, #4f4f4f2e 1px, transparent 1px),
                        linear-gradient(to bottom, #4f4f4f2e 1px, transparent 1px);
                        background-size: 14px 24px;"
            }

            // Hero content on top
            div {
                id: "hero",
                class: "relative z-10 flex p-4 justify-around mt-4 space-x-8",

                // left column of containers
                div { class: "flex flex-col space-y-4",

                    // text block container
                    div {
                        class: "p-4 bg-white border rounded-xl",
                         div { class:"prose",
                            h1 {
                                "About us",
                            }
                            p {
                                "Technology for Good (T4G) is dedicated to growing the responsible technology community through learning, career growth, and research enrichment. \
                                The organization hosts events centered on hands-on development, understanding the responsible technology landscape, and bettering understanding of research and graduate school opportunities.",
                            }
                        }
                    }
                    // text block container
                    div {
                        class: "p-4 bg-white border rounded-xl",
                        div { class:"prose",
                            h1 {
                                "Getting Involved",
                            }
                            p {
                                "T4G has weekly meetings that all Pitt students can attend to learn new things about using technology for good. \
                                Feel free to check out our discord and instagram to stay up to date on upcoming events!",
                            }
                        }
                    }
                }

                // Right column with stacked boxes
                div { class: "flex-col space-y-8",
                    div { class: "bg-white w-100 h-20 rounded-xl border" }
                    div { class: "bg-white w-100 h-20 rounded-xl border" }
                    div { class: "bg-white w-100 h-20 rounded-xl border" }
                    div { class: "bg-white w-100 h-20 rounded-xl border" }
                    div { class: "bg-white w-100 h-20 rounded-xl border" }
                    div { class: "bg-white w-100 h-20 rounded-xl border" }
                    div { class: "bg-white w-100 h-20 rounded-xl border" }
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
