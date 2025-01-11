#[cfg(test)]
mod tests {
    fn build_fragment(field: &str, operator: i8, value: Option<f32>) -> String {
        match value {
            None => "".to_string(),
            Some(value) => {
                let op = if value > 0 as f32 {
                    operator
                } else {
                    -operator
                };
                let word = if op > 0 { "greater" } else { "less" };

                format!("&filter={word}:{field}:{value}")
            }
        }
    }

    fn build_geo_query(criteria: &TrackCriteria) -> String {
        [
            build_fragment("latitude", 1, criteria.min_latitude_radians),
            build_fragment("latitude", -1, criteria.max_latitude_radians),
            build_fragment("longitude", 1, criteria.min_longitude_radians),
            build_fragment("longitude", -1, criteria.max_longitude_radians),
        ]
        .join("")
    }

    struct TrackCriteria {
        min_latitude_radians: Option<f32>,
        max_latitude_radians: Option<f32>,
        min_longitude_radians: Option<f32>,
        max_longitude_radians: Option<f32>,
    }

    #[test]
    fn test_greater_than_positive() {
        let fragment = build_fragment("latitude", 1, Some(0.11));
        assert_eq!(fragment, "&filter=greater:latitude:0.11");
    }

    #[test]
    fn test_greater_than_negative() {
        let fragment = build_fragment("latitude", 1, Some(-0.11));
        assert_eq!(fragment, "&filter=less:latitude:-0.11");
    }

    #[test]
    fn test_less_than_positive() {
        let fragment = build_fragment("latitude", -1, Some(0.11));
        assert_eq!(fragment, "&filter=less:latitude:0.11");
    }

    #[test]
    fn test_less_than_negative() {
        let fragment = build_fragment("latitude", -1, Some(-0.11));
        assert_eq!(fragment, "&filter=greater:latitude:-0.11");
    }

    #[test]
    fn test_all_none() {
        let criteria = TrackCriteria {
            min_longitude_radians: None,
            min_latitude_radians: None,
            max_latitude_radians: None,
            max_longitude_radians: None,
        };

        let query = build_geo_query(&criteria);
        assert_eq!(query, "");
    }

    #[test]
    fn test_mixed() {
        let criteria = TrackCriteria {
            min_longitude_radians: Some(1.1),
            min_latitude_radians: None,
            max_latitude_radians: None,
            max_longitude_radians: Some(-2.2),
        };

        let query = build_geo_query(&criteria);
        assert_eq!(
            query,
            "&filter=greater:longitude:1.1&filter=greater:longitude:-2.2"
        );
    }
}
