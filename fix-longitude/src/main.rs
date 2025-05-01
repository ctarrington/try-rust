fn main() {
    println!("Hello, world!");
    let range = convert_to_ranges(50.0, 60.0);
    println!("{:?}", range);
    let in_bounds = is_in_bounds(50.0, 60.0, 55.0);
    println!("{:?}", in_bounds);
}

type Ranges = [Option<(f64, f64)>; 2];

fn convert_to_ranges(left_longitude: f64, right_longitude: f64) -> Ranges {
    if left_longitude < right_longitude {
        [Some((left_longitude, right_longitude)), None::<(f64, f64)>]
    } else if left_longitude >= 0.0 && right_longitude < 0.0 {
        [
            Some((left_longitude, 180.0)),
            Some((-180.0, right_longitude)),
        ]
    } else {
        [Some((left_longitude, 180.0)), Some((0.0, right_longitude))]
    }
}

fn is_in_bounds(left_longitude: f64, right_longitude: f64, value: f64) -> bool {
    convert_to_ranges(left_longitude, right_longitude)
        .iter()
        .flatten()
        .any(|range| value > range.0 && value < range.1)
}

#[cfg(test)]
mod tests {
    use crate::{Ranges, convert_to_ranges, is_in_bounds};

    #[test]
    fn normal_values() {
        let [primary, secondary]: Ranges = convert_to_ranges(50.0, 60.0);
        assert_eq!(primary, Some((50.0, 60.0)));
        assert_eq!(secondary, None::<(f64, f64)>);
    }

    #[test]
    fn straddle_meridian() {
        let [primary, secondary]: Ranges = convert_to_ranges(170.0, 10.0);
        assert_eq!(primary, Some((170.0, 180.0)));
        assert_eq!(secondary, Some((0.0, 10.0)));
    }

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
        assert!(is_in_bounds(151.0, -177.0, 177.0));
        assert!(is_in_bounds(151.0, -177.0, -178.0));
    }

    #[test]
    fn straddle_out_of_bounds() {
        assert!(!is_in_bounds(170.0, 10.0, 15.0));
        assert!(!is_in_bounds(170.0, 10.0, 165.0));
        assert!(!is_in_bounds(-100.0, -80.0, -110.0));
        assert!(!is_in_bounds(-100.0, -80.0, -70.0));
        assert!(!is_in_bounds(-10.0, 10.0, 11.0));
        assert!(!is_in_bounds(151.0, -177.0, -167.0));
        assert!(!is_in_bounds(151.0, -177.0, 148.0));
    }
}
