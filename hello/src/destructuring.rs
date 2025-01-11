#[cfg(test)]
mod tests {
    struct Holder<T>(T);

    fn capitalize(Holder(value): Holder<String>) -> String {
        value.to_uppercase()
    }

    fn add_three(Holder(value): Holder<i32>) -> i32 {
        value + 3
    }

    #[test]
    fn test_destructure_assignment() {
        let held_thing = Holder("Thing".to_string());
        let Holder(thing) = held_thing;
        assert_eq!(thing, "Thing");

        let held_number = Holder(3);
        let Holder(value) = held_number;
        assert_eq!(3, value);
    }

    #[test]
    fn test_destructure_parameter() {
        let held_thing = Holder("Thing".to_string());
        assert_eq!(capitalize(held_thing), "THING");

        let held_number = Holder(10);
        assert_eq!(add_three(held_number), 13);
    }
}
