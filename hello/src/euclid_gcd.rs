#[cfg(test)]
mod tests {
    fn gcd(a: u32, b: u32) -> u32 {
        let min = if a <= b { a } else { b };
        let max = if b > a { b } else { a };

        let remainder = max % min;
        match remainder {
            0 => min,
            _ => gcd(min, remainder),
        }
    }

    #[test]
    fn simple() {
        assert_eq!(gcd(15, 6), 3);
    }

    #[test]
    fn same() {
        assert_eq!(gcd(3, 3), 3);
    }

    #[test]
    fn primes() {
        assert_eq!(gcd(7, 11), 1);
    }
}
