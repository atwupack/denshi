use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum EventValue {
    Clicked,
    ChildClicked(String),
    ValueChanged(String),
    Created,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    pub id: String,
    pub value: EventValue,
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
