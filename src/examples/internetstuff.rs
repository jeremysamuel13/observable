use std::{collections::HashMap, hash::Hash};

use crate::observable::*;
use macros::{event, observable};
use strum_macros::Display;

#[derive(Eq, PartialEq, Hash, Debug, Clone, Display)]
pub enum EventType {
    Connected,
    Disconnected,
    Waiting,
}

#[observable]
pub struct InternetStuff {
    observer_map: HashMap<String, CallbackType<fn(Event<&Self>), fn(Event<&mut Self>)>>,
    person_status_map: HashMap<String, EventType>,
    output: String,
}

impl std::fmt::Debug for InternetStuff {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InternetStuff")
            .field("person_status_map", &self.person_status_map)
            .finish()
    }
}

impl Observable<EventType> for InternetStuff {
    fn get_callback(
        &self,
        event: impl ToString,
    ) -> Option<CallbackType<fn(Event<&Self>), fn(Event<&mut Self>)>> {
        self.observer_map.get(&event.to_string()).cloned()
    }

    fn push_callback(
        &mut self,
        event: impl ToString,
        cb: CallbackType<fn(Event<&Self>), fn(Event<&mut Self>)>,
    ) {
        self.observer_map.insert(event.to_string(), cb);
    }

    fn remove_callback(&mut self, event: impl ToString) {
        self.observer_map.remove(&event.to_string());
    }
}

impl InternetStuff {
    pub fn new() -> Self {
        let mut val = Self {
            observer_map: HashMap::new(),
            person_status_map: HashMap::new(),
            output: "".to_string(),
        };

        val.on_mut(EventType::Connected, |evt| evt.src.output = evt.event);
        val.on_mut(EventType::Disconnected, |evt| evt.src.output = evt.event);
        val.on_mut(EventType::Waiting, |evt| evt.src.output = evt.event);

        val
    }

    pub fn with_callback(mut self, event: EventType, f: fn(Event<&Self>)) -> Self {
        self.on(event, f);
        self
    }

    pub fn with_callback_mut(mut self, event: EventType, f: fn(Event<&mut Self>)) -> Self {
        self.on_mut(event, f);
        self
    }

    #[event(name = "Connected")]
    pub fn connect(&mut self, person: impl Into<String>) {
        self.person_status_map
            .insert(person.into(), EventType::Connected);
    }

    #[event(name = "Waiting")]
    pub fn wait(&mut self, person: impl Into<String>) {
        self.person_status_map
            .insert(person.into(), EventType::Waiting);
    }

    #[event(name = "Disconnected")]
    pub fn disconnect(&mut self, person: impl Into<String>) {
        self.person_status_map
            .insert(person.into(), EventType::Disconnected);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lifecycle() {
        let mut it = InternetStuff::new()
            .with_callback_mut(EventType::Connected, |evt| evt.src.output = evt.event)
            .with_callback_mut(EventType::Disconnected, |evt| evt.src.output = evt.event)
            .with_callback_mut(EventType::Waiting, |evt| evt.src.output = evt.event);

        it.connect("Jeremy");

        assert_eq!(it.output, EventType::Connected.to_string());

        it.connect("John");

        assert_eq!(it.output, EventType::Connected.to_string());

        it.wait("John");

        assert_eq!(it.output, EventType::Waiting.to_string());

        it.disconnect("John");

        assert_eq!(it.output, EventType::Disconnected.to_string());

        it.wait("Jeremy");

        assert_eq!(it.output, EventType::Waiting.to_string());

        it.disconnect("Jeremy");

        assert_eq!(it.output, EventType::Disconnected.to_string());
    }
}