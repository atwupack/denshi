use serde::{Deserialize, Serialize};
use std::any::{Any, TypeId};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub enum EventValue {
    Clicked,
    ChildClicked(String),
    ValueChanged(String),
    Created,
    PageLoaded,
    NodeExpand(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    pub id: String,
    pub value: EventValue,
}

pub struct EventBroker {
    listeners: HashMap<TypeId, Vec<Box<dyn Fn(&dyn Any)>>>,
}

impl EventBroker {
    pub fn new() -> Self {
        EventBroker {
            listeners: HashMap::new(),
        }
    }

    pub fn send<E: Any>(&self, event: &E) {
        let type_id = TypeId::of::<E>();
        let listeners = self.listeners.get(&type_id);
        if let Some(l) = listeners {
            for item in l {
                item(event);
            }
        }
    }

    fn add_listener<F: Fn(&dyn Any) + 'static>(&mut self, type_id: TypeId, f: F) {
        let mut recvs = self.listeners.remove(&type_id).unwrap_or_default();
        recvs.push(Box::new(f));
        self.listeners.insert(type_id, recvs);
    }

    pub fn subscribe<F: Fn(&E) + 'static, E: Any>(&mut self, f: F) {
        let type_id = TypeId::of::<E>();
        self.add_listener(type_id, move |event| {
            let cast_message: &E = event.downcast_ref().unwrap();
            f(cast_message);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clicked() {
        let event = Event {
            id: "1234".into(),
            value: EventValue::Clicked,
        };

        dbg!(&event);

        dbg!(serde_json::to_string(&event).unwrap());
    }

    #[test]
    fn test_value_changed() {
        let event = Event {
            id: "1234".into(),
            value: EventValue::ValueChanged("Test".to_string()),
        };

        dbg!(&event);

        dbg!(serde_json::to_string(&event).unwrap());
    }

    #[test]
    fn test_value_created() {
        let event = Event {
            id: "1234".into(),
            value: EventValue::Created,
        };

        dbg!(&event);

        dbg!(serde_json::to_string(&event).unwrap());
    }
}
