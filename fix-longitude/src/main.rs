fn main() {
    println!("Hello, world!");
    let in_bounds = is_in_bounds(50.0, 60.0, 55.0);
    println!("{:?}", in_bounds);
}

fn is_in_bounds(left_longitude: f64, right_longitude: f64, value: f64) -> bool {
    let min = left_longitude.min(right_longitude);
    let max = left_longitude.max(right_longitude);
    let in_range = value >= min && value <= max;
    if left_longitude > right_longitude {
        !in_range
    } else {
        in_range
    }
}

#[cfg(test)]
mod tests {
    use crate::is_in_bounds;

    #[test]
    fn normal_in_bounds() {
        assert!(is_in_bounds(50.0, 60.0, 55.0));
    }

    #[test]
    fn normal_out_of_bounds() {
        assert!(!is_in_bounds(50.0, 60.0, 45.0));
        assert!(!is_in_bounds(50.0, 60.0, 65.0));
    }

    #[test]
    fn straddle_in_bounds() {
        assert!(is_in_bounds(170.0, 10.0, 175.0));
        assert!(is_in_bounds(170.0, 10.0, 5.0));
        assert!(is_in_bounds(-100.0, -80.0, -90.0));
        assert!(is_in_bounds(-10.0, 10.0, 1.0));
        assert!(is_in_bounds(151.0, -177.0, 179.9));
        assert!(is_in_bounds(151.0, -177.0, -178.0));
        assert!(is_in_bounds(151.0, -177.0, -180.0));
        assert!(is_in_bounds(151.0, -177.0, 180.0));
    }

    #[test]
    fn straddle_out_of_bounds() {
        assert!(!is_in_bounds(170.0, 10.0, 15.0));
        assert!(!is_in_bounds(170.0, 10.0, 165.0));
        assert!(!is_in_bounds(-100.0, -80.0, -110.0));
        assert!(!is_in_bounds(-100.0, -80.0, -70.0));
        assert!(!is_in_bounds(-10.0, 10.0, 11.0));
        assert!(!is_in_bounds(151.0, -177.0, -176.0));
        assert!(!is_in_bounds(151.0, -177.0, 150.9));
    }
}
