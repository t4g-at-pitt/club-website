use crate::components::event_list::EventList;
use crate::state::use_events_sorted;
use dioxus::prelude::*;
use crate::components::{footer::Footer, hero::Hero};

#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}
        Footer{}
    }
}
