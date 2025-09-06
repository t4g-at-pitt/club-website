use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The basic info card
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InfoCard {
    pub description: String,
}

/// Optional: a wrapper if you want to attach an ID (key) like your EventWithId
#[derive(Debug, Clone, PartialEq)]
pub struct InfoCardWithId {
    pub id: String,
    pub info_card: InfoCard,
}

impl InfoCardWithId {
    pub fn title(&self) -> &str {
        &self.id
    }

    pub fn description(&self) -> &str {
        &self.info_card.description
    }
}

/// Type alias for the JSON structure
pub type InfoCardsData = HashMap<String, InfoCard>;

/// Include your JSON file at compile time
static INFOCARDS_JSON: &str = include_str!("../../assets/data/info.json");

/// Parse and return all info cards with their IDs
pub fn get_info_cards() -> Result<Vec<InfoCardWithId>, serde_json::Error> {
    let map: InfoCardsData = serde_json::from_str(INFOCARDS_JSON)?;

    let cards = map
        .into_iter()
        .map(|(id, info_card)| InfoCardWithId { id, info_card })
        .collect();

    Ok(cards)
}

/// Get info cards, panicking on parse error (use in static contexts)
pub fn get_info_cards_static() -> Vec<InfoCardWithId> {
    get_info_cards().expect("Failed to parse info_cards.json")
}

/// Get a specific card by title
pub fn get_info_card_by_id(id: &str) -> Option<InfoCardWithId> {
    let map: InfoCardsData = serde_json::from_str(INFOCARDS_JSON)
        .expect("Failed to parse info_cards.json");

    map.get(id)
        .map(|info_card| InfoCardWithId {
            id: id.to_string(),
            info_card: info_card.clone(),
        })
}
