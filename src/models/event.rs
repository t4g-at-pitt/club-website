use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Event {
    pub name: String,
    pub date: String,
    pub location: String,
}

impl Event {
    /// Create a new Event with an ID (useful for iteration)
    pub fn with_id(self, id: String) -> EventWithId {
        EventWithId {
            id,
            event: self,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EventWithId {
    pub id: String,
    pub event: Event,
}

impl EventWithId {
    pub fn name(&self) -> &str {
        &self.event.name
    }

    pub fn date(&self) -> &str {
        &self.event.date
    }

    pub fn location(&self) -> &str {
        &self.event.location
    }

    pub fn is_tba(&self) -> bool {
        self.event.date.to_lowercase().contains("tba")
    }
}

// Type alias for the JSON structure
pub type EventsData = HashMap<String, Event>;

/// Include the JSON file at compile time
static EVENTS_JSON: &str = include_str!("../../assets/data/events.json");

/// Parse and return all events with their IDs
pub fn get_events() -> Result<Vec<EventWithId>, serde_json::Error> {
    let events_map: EventsData = serde_json::from_str(EVENTS_JSON)?;

    let events = events_map
        .into_iter()
        .map(|(id, event)| event.with_id(id))
        .collect();

    Ok(events)
}

/// Get events, panicking on parse error (use in static contexts)
pub fn get_events_static() -> Vec<EventWithId> {
    get_events().expect("Failed to parse events.json")
}

/// Get a specific event by ID
pub fn get_event_by_id(id: &str) -> Option<EventWithId> {
    let events_map: EventsData = serde_json::from_str(EVENTS_JSON)
        .expect("Failed to parse events.json");

    events_map
        .get(id)
        .map(|event| event.clone().with_id(id.to_string()))
}

/// Get events sorted by date (TBA events go last)
pub fn get_events_sorted() -> Vec<EventWithId> {
    let mut events = get_events_static();

    events.sort_by(|a, b| {
        match (a.is_tba(), b.is_tba()) {
            (true, false) => std::cmp::Ordering::Greater,  // TBA goes after dated events
            (false, true) => std::cmp::Ordering::Less,     // Dated events go before TBA
            _ => a.event.name.cmp(&b.event.name),          // Otherwise sort by name
        }
    });

    events
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_events() {
        let events = get_events().unwrap();
        assert!(events.len() >= 2);

        // Check that we have the expected events
        let event_ids: Vec<&str> = events.iter().map(|e| e.id.as_str()).collect();
        assert!(event_ids.contains(&"Zero2Paper"));
        assert!(event_ids.contains(&"GBM"));
    }

    #[test]
    fn test_get_event_by_id() {
        let gbm_event = get_event_by_id("GBM").unwrap();
        assert_eq!(gbm_event.name(), "GBM");
        assert_eq!(gbm_event.date(), "September 8th, 2025");
        assert_eq!(gbm_event.location(), "Snt. Square");
    }

    #[test]
    fn test_is_tba() {
        let zero2paper = get_event_by_id("Zero2Paper").unwrap();
        let gbm = get_event_by_id("GBM").unwrap();

        assert!(zero2paper.is_tba());
        assert!(!gbm.is_tba());
    }

    #[test]
    fn test_sorted_events() {
        let sorted = get_events_sorted();
        // Should have GBM first (has date), Zero2Paper last (TBA)
        assert_eq!(sorted[0].id, "GBM");
        assert_eq!(sorted.last().unwrap().id, "Zero2Paper");
    }
}