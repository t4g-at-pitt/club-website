use dioxus::prelude::*;
#[component]
pub fn Footer() -> Element {
    rsx! {
        div { class:" p-4 h-[30dvh] bg-green-200",
            footer { class:"w-lg ml-16 justify-center",
                h2 { class:"text-3xl mb-4 font-bold", "Socials" }
                ul { class:" gap-y-1 flex flex-col",
                    li { class:"flex items-center text-xl font-medium",
                        img { class:"mr-2 size-5", src:asset!("/assets/icons/linkedin.svg") }
                        span { "LinkedIn" }
                    }
                    li { class:"flex items-center text-xl font-medium",
                        img { class:"mr-2 size-5", src:asset!("/assets/icons/instagram.svg") }
                        span { "Instagram" }
                    }
                }
            }
        }
    }
}
