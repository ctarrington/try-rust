pub mod observations {
    pub struct System {
        pub guid: uuid::Uuid,
        pub name: String,
        pub description: String,
    }

    pub enum Observer {
        Local(System),
        Peer(System),
        Other(System),
    }

    /*
    Thing - a real world thing that can be observed. It has a guid and a name.
     */
    pub struct Thing {
        pub guid: uuid::Uuid,
        pub name: String,
    }

    /*
    Observation - a record of an observation of a thing.
    It has a guid, a thing guid, a timestamp, and some values.
     */
    pub struct Observation {
        pub guid: uuid::Uuid,
        pub observer: Observer,
        pub thing: Thing,
        pub timestamp: chrono::DateTime<chrono::Utc>,
        pub values: Vec<f64>,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use observations::*;

    #[test]
    fn it_works() {
        let system = System {
            guid: uuid::Uuid::new_v4(),
            name: "Test System".to_string(),
            description: "Just a test system".to_string(),
        };

        assert_eq!(system.name, "Test System");
        assert_eq!(system.description, "Just a test system");

        let thing = Thing {
            guid: uuid::Uuid::new_v4(),
            name: "Test Thing".to_string(),
        };

        assert_eq!(thing.name, "Test Thing");

        let timestamp = chrono::Utc::now();
        let observation = Observation {
            guid: uuid::Uuid::new_v4(),
            observer: Observer::Local(system),
            thing,
            timestamp,
            values: vec![1.0, 2.0, 3.0],
        };

        let system = match observation.observer {
            Observer::Local(system) => system,
            _ => panic!("Expected Observer::Local"),
        };
        assert_eq!(system.name, "Test System");

        assert_eq!(observation.thing.name, "Test Thing");
        assert_eq!(observation.timestamp, timestamp);
        assert_eq!(observation.values[0], 1.0);
        assert_eq!(observation.values.len(), 3);
    }
}
