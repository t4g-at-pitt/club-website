use crate::components::event_card::EventCard;
use crate::state::use_events_sorted;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;

#[component]
pub fn EventList() -> Element {
    let events = use_events_sorted();
    let upcoming_count = events.read().iter().filter(|e| !e.is_tba()).count();
    let tba_count = events.read().iter().filter(|e| e.is_tba()).count();

    rsx! {
        div { class:"flex flex-col",
            header {
            class: "mb-4",
            h1 {
                class: "text-left text-5xl font-bold text-gray-900 mb-4",
                "Upcoming Events"
            }
            div { class: "flex space-x-4 text-md text-gray-600",
                span {
                    "🗓️ {upcoming_count} scheduled"
                }
                {(tba_count > 0).then(|| rsx! {
                    span {
                        "⏰ {tba_count} to be announced"
                    }
                })}
            }
        }
        main {
            div { class: "flex flex-col space-y-4",
                for event in events.read().iter() {
                    EventCard { event: event.clone() }
                }
            }
        }

        }

    }
}

// Version with error handling
#[component]
fn EventListWithErrorHandling() -> Element {
    // If you want to handle potential parsing errors gracefully
    let events_result = use_memo(|| crate::models::event::get_events_sorted());

    let events = events_result();

    rsx! {
        div { class: "flex flex-col space-y-4",
            if events.is_empty() {
                div { class: "text-center py-8 text-gray-500",
                    p { "No events scheduled at the moment." }
                    p { class: "text-sm mt-2",
                        "Check back soon for updates!"
                    }
                }
            } else {
                for event in events.iter() {
                    EventCard { event: event.clone() }
                }
            }
        }
    }
}

// Alternative version if you want to separate upcoming and TBA events
#[component]
fn EventListSeparated() -> Element {
    let events = use_events_sorted();

    let upcoming_events: Vec<_> = events
        .read()
        .iter()
        .filter(|e| !e.is_tba())
        .cloned()
        .collect();

    let tba_events: Vec<_> = events
        .read()
        .iter()
        .filter(|e| e.is_tba())
        .cloned()
        .collect();

    rsx! {
        div { class: "space-y-8",
            // Upcoming Events Section
            {(!upcoming_events.is_empty()).then(|| rsx! {
                section {
                    h2 { class: "text-3xl font-semibold text-gray-800 mb-4",
                        "Scheduled Events"
                    }
                    div { class: "flex flex-col space-y-4",
                        for event in upcoming_events.iter() {
                            EventCard { event: event.clone() }
                        }
                    }
                }
            })}

            // TBA Events Section
            {(!tba_events.is_empty()).then(|| rsx! {
                section {
                    h2 { class: "text-2xl font-semibold text-gray-800 mb-4",
                        "Coming Soon"
                    }
                    p { class: "text-gray-600 mb-4 text-sm",
                        "These events are being planned - dates will be announced soon!"
                    }
                    div { class: "flex flex-col space-y-4",
                        for event in tba_events.iter() {
                            EventCard { event: event.clone() }
                        }
                    }
                }
            })}
        }
    }
}
