#[cfg(test)]
mod tests {
    struct Widget {
        size: usize,
        color: (u8, u8, u8),
    }

    struct Holder<T>(T);

    fn capitalize(Holder(value): Holder<String>) -> String {
        value.to_uppercase()
    }

    fn add_three(Holder(value): Holder<i32>) -> i32 {
        value + 3
    }

    fn debug_widget(
        Holder(Widget {
            size,
            color: (r, g, b),
        }): Holder<Widget>,
    ) -> String {
        format!("size: {size}, rgb: ({r}, {g}, {b})")
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

        let held_widget = Holder(Widget {
            size: 1,
            color: (2, 4, 6),
        });
        assert_eq!(debug_widget(held_widget), "size: 1, rgb: (2, 4, 6)")
    }

    #[test]
    fn test_matches() {
        let held_widget = Holder(Widget {
            size: 1,
            color: (2, 4, 6),
        });

        let rising = matches!(
            held_widget,
            Holder(Widget {
                color: (r, g, b),..
            }) if r < g && g < b
        );
        assert!(rising);

        let small = matches!(held_widget, Holder(Widget {size, ..}) if size < 2);
        assert!(small);

        let big_widget = Holder(Widget {
            size: 10,
            color: (1, 2, 3),
        });
        let small = matches!(big_widget, Holder(Widget {size, ..}) if size < 2);
        assert!(!small);
    }
}
