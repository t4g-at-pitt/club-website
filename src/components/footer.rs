use dioxus::prelude::*;
#[component]
pub fn Footer() -> Element {
    rsx! {
        div { class:"relative p-10 bg-green-100 bg-[radial-gradient(theme(colors.blue.200)_1px,transparent_1px)] [background-size:16px_16px] flex justify-left items-center border-t border-gray-500",
            footer {
                // link to source code
                a {
                    href: "https://github.com/t4g-at-pitt/club-website",
                    target: "_blank",
                    code { class:"text-sm absolute right-6 bottom-6 opacity-50 hover:opacity-80",
                        "source code"
                    }
                }
                ul { class:"gap-y-1 flex flex-col w-fit",
                    h2 { class:"text-3xl mb-4 font-bold", "Connect with us!" }
                    li {
                        a { class:"w-fit hover:cursor-pointer flex flex-row items-center text-xl font-medium hover:underline",
                            target:"_blank",
                            alt:"T4G LinkedIn",
                            href:"https://www.linkedin.com/in/technology-for-good-at-pitt-924936347",
                            img {
                                class:"mr-2 size-5",
                                src:asset!("/assets/icons/linkedin.svg")
                            }
                            span { "LinkedIn" }
                        }
                    }
                    li {
                        a { class:"w-fit hover:cursor-pointer flex flex-row items-center text-xl font-medium hover:underline",
                            target:"_blank",
                            alt:"T4G Instagram",
                            href:"https://www.instagram.com/t4g_at_pitt/",
                            img { class:"mr-2 size-5", src:asset!("/assets/icons/instagram.svg") }
                            span { "Instagram" }
                        }
                    }
                    li {
                        a { class:"w-fit hover:cursor-pointer flex flex-row items-center text-xl font-medium hover:underline",
                            target:"_blank",
                            href:"https://discord.gg/7ZzZ7PmaXc",
                            alt:"T4G Discord",
                            img { class:"mr-2 size-5", src:asset!("/assets/icons/discord.svg") }
                            span { "Discord" }
                        }
                    }
                    li {
                        a { class:"w-fit hover:cursor-pointer flex flex-row items-center text-xl font-medium hover:underline",
                            target:"_blank",
                            alt:"T4G GitHub Organization",
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
