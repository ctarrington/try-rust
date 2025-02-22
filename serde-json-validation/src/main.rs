use serde::{Deserialize, Serialize};

fn main() {
    println!("Hello, world!");
}

#[derive(Serialize, Deserialize)]
struct Thing {
    name: String,
    height: u32,
}

#[derive(Serialize, Deserialize)]
struct ThingResult {
    count: u32,
    things: Vec<Thing>,
}

#[cfg(test)]
mod tests {
    use crate::ThingResult;

    #[test]
    fn present() {
        let raw = r#"
            {
            "count":2,
            "things": [{
                "name": "fred",
                "height": 12
              }, {
                "name": "ted",
                "height": 14
              }]
            }
            "#;

        let thing_result: serde_json::error::Result<ThingResult>= serde_json::from_str(raw);
        if let Ok(thing_result) = thing_result {
            assert_eq!(2, thing_result.count);
            assert_eq!(2, thing_result.things.len());
        } else {
            assert!(false);
        }
    }
}
