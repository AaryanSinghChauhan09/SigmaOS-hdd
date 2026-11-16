// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-hdd: hdd component model.
//!
//! Real implementation: hdd_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HddItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HddError {
    EmptyHddItemName,
    DuplicateHddItem(String),
    EmptyEventSubject,
}

impl fmt::Display for HddError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyHddItemName => write!(f, "hdd_item name must not be empty"),
            Self::DuplicateHddItem(id) => write!(f, "duplicate hdd_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for HddError {}

pub struct Hdd {
    pub name: String,
    hdd_items: BTreeMap<String, HddItem>,
    events: BTreeMap<String, Event>,
}

impl Hdd {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            hdd_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_hdd_item(&mut self, item: HddItem) -> Result<(), HddError> {
        if item.name.trim().is_empty() {
            return Err(HddError::EmptyHddItemName);
        }
        let id = item.id.clone();
        if self.hdd_items.contains_key(&id) {
            return Err(HddError::DuplicateHddItem(id.clone()));
        }
        self.hdd_items.insert(id, item);
        Ok(())
    }

    pub fn hdd_item_count(&self) -> usize {
        self.hdd_items.len()
    }

    pub fn get_hdd_item(&self, id: &str) -> Option<&HddItem> {
        self.hdd_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), HddError> {
        if event.subject.trim().is_empty() {
            return Err(HddError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_hdd_item(id: &str, name: &str) -> HddItem {
        HddItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Hdd::new("Hdd");
        assert_eq!(m.name, "Hdd");
        assert_eq!(m.hdd_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_hdd_item() {
        let mut m = Hdd::new("Hdd");
        m.add_hdd_item(sample_hdd_item("a1", "Alpha")).unwrap();
        assert_eq!(m.hdd_item_count(), 1);
        assert!(m.get_hdd_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Hdd::new("Hdd");
        assert!(m.add_hdd_item(sample_hdd_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Hdd::new("Hdd");
        m.add_hdd_item(sample_hdd_item("x", "One")).unwrap();
        assert!(m.add_hdd_item(sample_hdd_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Hdd::new("Hdd");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
