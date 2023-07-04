fn shift(digit: u32) -> u32 {
    let value = 2* digit;
    if value < 10 {
        return value;
    }

    let first_digit = value / 10;
    let second_digit = value % 10;
    first_digit + second_digit
}

fn convert(cc_number: &str) -> Vec<u32> {
    let mut cc_number = String::from(cc_number);
    cc_number.retain(|c| c != ' ');
    let mut numbers: Vec<u32> = Vec::new();

    for (index,character) in cc_number.chars().rev().enumerate() {
        if let Some(value)  = character.to_digit(10) {
            if index % 2 == 0 {
                numbers.push(value);
            } else {
                numbers.push(shift(value));
            }
        }
    }

    numbers.reverse();
    numbers
}

pub fn luhn(cc_number: &str) -> bool {
    let numbers = convert(cc_number);

    if numbers.len() < 2 {
        return false;
    }

    let sum: u32 = numbers.iter().sum();
    sum % 10 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shifts() {
        assert_eq!(shift(0), 0);
        assert_eq!(shift(1), 2);
        assert_eq!(shift(2), 4);
        assert_eq!(shift(3), 6);
        assert_eq!(shift(4), 8);
        assert_eq!(shift(5), 1);
        assert_eq!(shift(6), 3);
        assert_eq!(shift(7), 5);
        assert_eq!(shift(8), 7);
        assert_eq!(shift(9), 9);
    }

    #[test]
    fn test_non_digit_cc_number() {
        assert!(!luhn("foo"));
    }

    #[test]
    fn test_two_digit_cc_number() {
        assert!(luhn(" 0 0 "));
    }

    #[test]
    fn test_convert() {
        assert_eq!(convert("4263 9826 4026 9299"), vec![8,2,3,3,  9,8,4,6,  8,0,4,6,  9,2,9,9]);
    }

    #[test]
    fn test_valid_cc_number() {
        assert!(luhn("4263 9826 4026 9299"));
        assert!(luhn("4539 3195 0343 6467"));
        assert!(luhn("7992 7398 713"));
    }

    #[test]
    fn test_invalid_cc_number() {
        assert!(!luhn("4223 9826 4026 9299"));
        assert!(!luhn("4539 3195 0343 6476"));
        assert!(!luhn("8273 1232 7352 0569"));
    }
}
