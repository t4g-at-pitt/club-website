use dioxus::prelude::*;
use crate::models::event::{EventWithId, get_events_static, get_events_sorted};
use crate::models::info_card::{InfoCardWithId, get_info_cards_static};


// Option 1: Using Dioxus signals (recommended for Dioxus 0.5+)
pub fn use_events() -> Signal<Vec<EventWithId>> {
    use_signal(|| get_events_static())
}

pub fn use_events_sorted() -> Signal<Vec<EventWithId>> {
    use_signal(|| get_events_sorted())
}

// --- Option 1: Using Dioxus signals ---
pub fn use_info_cards() -> Signal<Vec<InfoCardWithId>> {
    use_signal(|| get_info_cards_static())
}
