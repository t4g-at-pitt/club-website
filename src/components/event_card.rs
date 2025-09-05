use dioxus::prelude::*;
use crate::models::event::EventWithId;

#[component]
pub fn EventCard(event: EventWithId) -> Element {
    let date_class = if event.is_tba() {
        "text-gray-500 italic"
    } else {
        "text-gray-700"
    };

    rsx! {
        div {
            key: "{event.id}",
            class: "p-4 bg-white border rounded-xl flex flex-col shadow-sm hover:shadow-md transition-shadow",

            div { class: "flex items-center justify-between mb-2",
                strong { class: "text-lg font-semibold text-gray-900",
                    "{event.name()}"
                }
                {event.is_tba().then(|| rsx! {
                    span { class: "px-2 py-1 text-xs bg-yellow-100 text-yellow-800 rounded-full",
                        "TBA"
                    }
                })}
            }

            div { class: "flex flex-col space-y-1",
                span { class: "flex items-center {date_class}",
                    "📅 {event.date()}"
                }
                span { class: "flex items-center text-gray-600",
                    "📍 {event.location()}"
                }
            }
        }
    }
}