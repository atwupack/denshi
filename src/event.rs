use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum EventValue {
    Click,
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
    fn test_click() {
        let event = Event {
            id: "1234".into(),
            value: EventValue::Click,
        };

        dbg!(&event);

        dbg!(serde_json::to_string(&event).unwrap());
    }


}