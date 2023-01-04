use std::collections::HashMap;

use macros::{event, observable};
use observable::*;
use strum_macros::Display;

#[derive(Eq, PartialEq, Hash, Debug, Clone, Display)]
pub enum EventType {
    Connected,
    Disconnected,
    Waiting,
    #[strum(to_string = "Error")] //so all errors fire on "Error"
    Error(String),
}

#[observable]
pub struct InternetStuff {
    observer_map:
        HashMap<String, CallbackType<fn(Event<EventType, &Self>), fn(Event<EventType, &mut Self>)>>,
    pub person_status_map: HashMap<String, EventType>,
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
    ) -> Option<CallbackType<fn(Event<EventType, &Self>), fn(Event<EventType, &mut Self>)>> {
        self.observer_map.get(&event.to_string()).cloned()
    }

    fn push_callback(
        &mut self,
        event: impl ToString,
        cb: CallbackType<fn(Event<EventType, &Self>), fn(Event<EventType, &mut Self>)>,
    ) {
        self.observer_map.insert(event.to_string(), cb);
    }

    fn remove_callback(&mut self, event: impl ToString) {
        self.observer_map.remove(&event.to_string());
    }
}

impl InternetStuff {
    pub fn new() -> Self {
        Self {
            observer_map: HashMap::new(),
            person_status_map: HashMap::new(),
            output: "".to_string(),
        }
    }

    pub fn with_callback(mut self, event: impl ToString, f: fn(Event<EventType, &Self>)) -> Self {
        self.on(event, f);
        self
    }

    pub fn with_callback_mut(
        mut self,
        event: impl ToString,
        f: fn(Event<EventType, &mut Self>),
    ) -> Self {
        self.on_mut(event, f);
        self
    }

    #[event(name = "Connected", return)]
    pub fn connect(&mut self, person: impl Into<String>) {
        self.person_status_map
            .insert(person.into(), EventType::Connected);
        Return::new(EventType::Connected, ()) //turns into `return ();`, and dispatches event to callbacks
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

    #[event(name = "Error", return)]
    pub fn error(&mut self, person: impl Into<String>, error: impl Into<String>) {
        let str: String = person.into();
        let error: String = error.into();
        let evt = EventType::Error(error.clone());
        self.person_status_map.insert(str.clone(), evt.clone());
        Return::new(evt, ()) //turns into `return ();`, and dispatches event to callbacks
    }
}

fn main() {
    fn assert_output(it: &InternetStuff, value: EventType) {
        let str = match value {
            EventType::Error(err) => format!("Error('{}')", err),
            val => val.to_string(),
        };

        assert_eq!(it.output, str);

        // log stuff as well

        println!("Event fired: {}", str);
    }

    let mut it = InternetStuff::new()
        .with_callback_mut(EventType::Connected, |evt| evt.src.output = evt.event_name)
        .with_callback_mut(EventType::Disconnected, |evt| {
            evt.src.output = evt.event_name
        })
        .with_callback_mut(EventType::Waiting, |evt| evt.src.output = evt.event_name)
        .with_callback_mut(EventType::Error("test".to_string()).to_string(), |evt| {
            if let Some(EventType::Error(err)) = evt.event {
                evt.src.output = format!("Error('{}')", err)
            }
        });

    let person = "🦀";

    it.connect(person);

    assert_output(&it, EventType::Connected);

    it.wait(person);

    assert_output(&it, EventType::Waiting);

    it.disconnect(person);

    assert_output(&it, EventType::Disconnected);

    it.error(person, "Error 1");

    assert_output(&it, EventType::Error("Error 1".to_string()));

    it.error(person, "Error 2");

    assert_output(&it, EventType::Error("Error 2".to_string()));

    it.error(person, "Error 5");

    assert_output(&it, EventType::Error("Error 5".to_string()));
}
