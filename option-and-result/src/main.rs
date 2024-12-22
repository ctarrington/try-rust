fn main() {
    println!("Hello, world!");
}

fn capitalize(word: &str) -> Option<String> {
    Some(word.to_uppercase())
}

fn process(word: &str) -> Option<&str> {
    match word {
        "Skip" => None,
        word => Some(word),
    }
}

// flatten drops out any Nones
fn capitalize_all(words: Vec<Option<&str>>) -> Vec<String> {
    words
        .iter()
        .map(|w| w.and_then(process).and_then(capitalize))
        .flatten()
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{capitalize, capitalize_all, process};

    #[test]
    fn skips() {
        let word = Some("Skip");
        assert_eq!(word.and_then(process).and_then(capitalize), None);
    }

    #[test]
    fn caps() {
        let word = Some("Hi");
        assert_eq!(
            word.and_then(process).and_then(capitalize),
            Some("HI".to_string())
        );
    }

    #[test]
    fn all_there() {
        let words = vec![Some("hi"), Some("there")];
        let result = capitalize_all(words);
        assert_eq!(result, vec!["HI", "THERE"]);
    }

    #[test]
    fn some_there() {
        let words = vec![Some("hi"), None, Some("there")];
        let result = capitalize_all(words);
        assert_eq!(result, vec!["HI", "THERE"]);
    }

    #[test]
    fn skip_there() {
        let words = vec![Some("hi"), Some("Skip"), Some("there")];
        let result = capitalize_all(words);
        assert_eq!(result, vec!["HI", "THERE"]);
    }

    #[test]
    fn none_there() {
        let empty: Vec<String> = vec![];
        let words = vec![None, Some("Skip")];
        let result = capitalize_all(words);
        assert_eq!(result, empty);
    }

    #[test]
    fn test_process() {
        assert_eq!(process("Skip"), None);
        assert_eq!(process("hi"), Some("hi"));
    }
}
