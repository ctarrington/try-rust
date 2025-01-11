#[cfg(test)]
mod tests {
    struct Holder<T>(T);

    fn capitalize_holder(Holder(value): Holder<String>) -> String {
        value.to_uppercase()
    }

    #[test]
    fn test_destructure_assignment() {
        let held_thing = Holder("Thing".to_string());
        let Holder(thing) = held_thing;
        assert_eq!(thing, "Thing");
    }

    #[test]
    fn test_destructure_parameter() {
        let held_thing = Holder("Thing".to_string());
        let result = capitalize_holder(held_thing);
        assert_eq!(result, "THING");
    }
}
