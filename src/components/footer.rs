use dioxus::prelude::*;
#[component]
pub fn Footer() -> Element {
    rsx! {
        div { class:"p-10 bg-green-100 flex justify-left items-center border-t border-gray-500",
            footer {
                ul { class:"gap-y-1 flex flex-col w-fit",
                    h2 { class:"text-3xl mb-4 font-bold", "Connect with us!" }
                    li {
                        a { class:"w-fit hover:cursor-pointer flex flex-row items-center text-xl font-medium hover:underline",
                            target:"_blank",
                            href:"https://www.linkedin.com/in/technology-for-good-at-pitt-924936347",
                            img { class:"mr-2 size-5", src:asset!("/assets/icons/linkedin.svg") }
                            span { "LinkedIn" }
                        }
                    }
                    li {
                        a { class:"w-fit hover:cursor-pointer flex flex-row items-center text-xl font-medium hover:underline",
                            target:"_blank",
                            href:"https://www.instagram.com/t4g_at_pitt/",
                            img { class:"mr-2 size-5", src:asset!("/assets/icons/instagram.svg") }
                            span { "Instagram" }
                        }
                    }
                    li {
                        a { class:"w-fit hover:cursor-pointer flex flex-row items-center text-xl font-medium hover:underline",
                            target:"_blank",
                            href:"https://discord.gg/7ZzZ7PmaXc",
                            img { class:"mr-2 size-5", src:asset!("/assets/icons/discord.svg") }
                            span { "Discord" }
                        }
                    }
                    li {
                        a { class:"w-fit hover:cursor-pointer flex flex-row items-center text-xl font-medium hover:underline",
                            target:"_blank",
                            href:"https://github.com/t4g-at-pitt",
                            img { class:"mr-2 size-5", src:asset!("/assets/icons/github.svg") }
                            span { "GitHub" }
                        }
                    }
                }
            }
        }
    }
}
