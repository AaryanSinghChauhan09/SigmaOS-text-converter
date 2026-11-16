// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-text-converter: text converter component model.
//!
//! Real implementation: texts, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Text {
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
pub enum TextConverterError {
    EmptyTextName,
    DuplicateText(String),
    EmptyEventSubject,
}

impl fmt::Display for TextConverterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyTextName => write!(f, "text name must not be empty"),
            Self::DuplicateText(id) => write!(f, "duplicate text {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for TextConverterError {}

pub struct TextConverter {
    pub name: String,
    texts: BTreeMap<String, Text>,
    events: BTreeMap<String, Event>,
}

impl TextConverter {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            texts: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_text(&mut self, item: Text) -> Result<(), TextConverterError> {
        if item.name.trim().is_empty() {
            return Err(TextConverterError::EmptyTextName);
        }
        let id = item.id.clone();
        if self.texts.contains_key(&id) {
            return Err(TextConverterError::DuplicateText(id.clone()));
        }
        self.texts.insert(id, item);
        Ok(())
    }

    pub fn text_count(&self) -> usize {
        self.texts.len()
    }

    pub fn get_text(&self, id: &str) -> Option<&Text> {
        self.texts.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), TextConverterError> {
        if event.subject.trim().is_empty() {
            return Err(TextConverterError::EmptyEventSubject);
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

    fn sample_text(id: &str, name: &str) -> Text {
        Text {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = TextConverter::new("TextConverter");
        assert_eq!(m.name, "TextConverter");
        assert_eq!(m.text_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_text() {
        let mut m = TextConverter::new("TextConverter");
        m.add_text(sample_text("a1", "Alpha")).unwrap();
        assert_eq!(m.text_count(), 1);
        assert!(m.get_text("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = TextConverter::new("TextConverter");
        assert!(m.add_text(sample_text("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = TextConverter::new("TextConverter");
        m.add_text(sample_text("x", "One")).unwrap();
        assert!(m.add_text(sample_text("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = TextConverter::new("TextConverter");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
