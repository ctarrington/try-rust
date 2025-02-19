#[cfg(test)]
mod tests {

    #[derive(Debug, Eq, PartialEq)]
    struct Thing {
        name: String,
        id: i32,
    }

    fn create_things() -> Vec<Thing> {
        (0..10)
            .into_iter()
            .map(|v| Thing {
                id: v,
                name: format!("Thing {v}"),
            })
            .collect()
    }

    fn mutate_things(source_of_things: impl IntoIterator<Item = Thing>) -> Vec<Thing> {
        source_of_things
            .into_iter()
            .map(|t| Thing {
                name: t.name.to_uppercase(),
                ..t
            })
            .collect()
    }

    #[test]
    fn upper_thing() {
        let things = create_things();
        let mutants = mutate_things(things);

        let mutant = mutants.get(0);
        assert!(matches!(mutant, Some(_)));
        let mutant = mutant.unwrap();

        assert_eq!(
            mutant,
            &Thing {
                name: "THING 0".to_string(),
                id: 0,
            }
        );
    }
}
