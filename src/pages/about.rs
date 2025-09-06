use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;

#[component]
pub fn About () -> Element {
    rsx! {
        div { class:"bg-yellow-50 h-full p-10",
            div { class:"grid grid-cols-2 gap-4 justify-around",


                div {
                    class: "prose p-4 max-w-xs md:max-w-xl bg-white border rounded-xl shadow-sm hover:shadow-md transition-shadow",
                    h1{
                        "Our values"
                    }
                    p {
                        "With a world of advancing technology, our club hopes to offer resources that align with  responsible techonolgoy."
                    }
                }
                div {
                    class: "prose p-4 max-w-xs md:max-w-xl bg-white border rounded-xl shadow-sm hover:shadow-md transition-shadow",
                    h1{
                        "Getting involved"
                    }
                    p {
                        "Our club is open to all members of the Pitt community. \
                        We host in-person meetings once a week, and provide online projects to contribute to year round. \
                        Feel free to visit our next meeting to meet people and learn something new."
                    }
                }
                div {
                    class: "prose p-4 max-w-xs md:max-w-xl bg-white border rounded-xl shadow-sm hover:shadow-md transition-shadow",
                    h1{
                        "Research"
                    }
                    p {
                        "Through workshops and online resources, T4G is a hub for resources related to getting involved with research and graduate school.\
                        "
                    }
                }
                div {
                    class: "prose p-4 max-w-xs md:max-w-xl bg-white border rounded-xl shadow-sm hover:shadow-md transition-shadow",
                    h1{
                        "Projects"
                    }
                    p {
                        "Our club hosts multiple repositories on GitHub, a platform for collaborating. \
                         We offer ethical communitive projects that anyone can actively help design or code.
                        "
                    }
                }
            }
            
        }
    }
}