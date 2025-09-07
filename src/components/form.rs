use dioxus::prelude::*;

#[component]
pub fn Form() -> Element {
    rsx! {
        div { class:"flex flex-col gap-y-3 max-w-2xl mx-auto justify-center prose",
            h1 { "Have any questions or suggestions?" }
            h3 { "Feel free to contact our team here." }
            form {
                class:"flex flex-col gap-y-3",
                action: "https://api.web3forms.com/submit",
                method: "POST",
                input {
                    r#type: "hidden",
                    name: "access_key",
                    value: "e82da891-cc22-419b-99c9-81798285f942",
                }
                input {
                    r#type: "text",
                    name: "name",
                    required: true,
                    placeholder: "Your Name",
                    class: "border p-2 rounded bg-white"
                }
                input {
                    r#type: "email",
                    name: "email",
                    required: true,
                    placeholder: "Your Email",
                    class: "border p-2 rounded bg-white"
                }
                textarea {
                    name: "message",
                    required: true,
                    placeholder: "Your Message",
                    class: "border p-2 rounded bg-white"
                }
                input {
                    r#type: "checkbox",
                    name: "botcheck",
                    class: "hidden",
                    style: "display: none;"
                }
                button {
                    r#type: "submit",
                    class: "mt-4 px-4 py-2 bg-green-600 text-white rounded hover:bg-green-700",
                    "Submit Form"
                }
            }
        }
    }
}
