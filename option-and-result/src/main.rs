fn main() {
    println!("Hello, world!");
}

fn capitalize(words: Vec<Option<String>>) -> Vec<String> {
    words
        .iter()
        .map(|w| w.clone().and_then(|w| Some(w.to_uppercase())))
        .flatten()
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::capitalize;

    #[test]
    fn all_there() {
        let words = vec![Some("hi".to_string()), Some("there".to_string())];
        let result = capitalize(words);
        assert_eq!(result, vec!["HI".to_string(), "THERE".to_string()]);
    }

    #[test]
    fn some_there() {
        let words = vec![Some("hi".to_string()), None, Some("there".to_string())];
        let result = capitalize(words);
        assert_eq!(result, vec!["HI".to_string(), "THERE".to_string()]);
    }

    #[test]
    fn none_there() {
        let empty: Vec<String> = vec![];
        let words = vec![None, None];
        let result = capitalize(words);
        assert_eq!(result, empty);
    }
}
