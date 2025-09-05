use dioxus::prelude::*;
use crate::models::event::{EventWithId, get_events_static, get_events_sorted};

// Option 1: Using Dioxus signals (recommended for Dioxus 0.5+)
pub fn use_events() -> Signal<Vec<EventWithId>> {
    use_signal(|| get_events_static())
}

pub fn use_events_sorted() -> Signal<Vec<EventWithId>> {
    use_signal(|| get_events_sorted())
}

// Option 2: Using context (good for sharing across many components)
#[derive(Clone)]
pub struct EventsContext {
    pub events: Vec<EventWithId>,
}

impl EventsContext {
    pub fn new() -> Self {
        Self {
            events: get_events_static(),
        }
    }

    pub fn get_event_by_id(&self, id: &str) -> Option<&EventWithId> {
        self.events.iter().find(|event| event.id == id)
    }

    pub fn get_upcoming_events(&self) -> Vec<&EventWithId> {
        self.events
            .iter()
            .filter(|event| !event.is_tba())
            .collect()
    }

    pub fn get_tba_events(&self) -> Vec<&EventWithId> {
        self.events
            .iter()
            .filter(|event| event.is_tba())
            .collect()
    }
}

// Option 3: Global static (simplest approach)
use std::sync::OnceLock;

static GLOBAL_EVENTS: OnceLock<Vec<EventWithId>> = OnceLock::new();

pub fn get_global_events() -> &'static Vec<EventWithId> {
    GLOBAL_EVENTS.get_or_init(|| get_events_static())
}