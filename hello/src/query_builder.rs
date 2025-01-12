#[cfg(test)]
mod tests {
    use crate::query_builder::tests::Operator::{Greater, Less};

    #[derive(Debug)]
    enum Operator {
        Greater,
        Less,
    }

    impl Operator {
        fn flip(self) -> Operator {
            match self {
                Greater => Less,
                Less => Greater,
            }
        }
    }

    fn build_fragment(field: &str, operator: Operator, value: Option<f32>) -> String {
        match value {
            None => "".to_string(),
            Some(value) => {
                let operator = if value > 0 as f32 {
                    operator
                } else {
                    operator.flip()
                };
                let operator = format!("{operator:?}").to_lowercase();
                format!("&filter={operator}:{field}:{value}")
            }
        }
    }

    fn build_geo_query(criteria: &TrackCriteria) -> String {
        [
            build_fragment("latitude", Greater, criteria.min_latitude_radians),
            build_fragment("latitude", Less, criteria.max_latitude_radians),
            build_fragment("longitude", Greater, criteria.min_longitude_radians),
            build_fragment("longitude", Less, criteria.max_longitude_radians),
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
        let fragment = build_fragment("latitude", Greater, Some(0.11));
        assert_eq!(fragment, "&filter=greater:latitude:0.11");
    }

    #[test]
    fn test_greater_than_negative() {
        let fragment = build_fragment("latitude", Greater, Some(-0.11));
        assert_eq!(fragment, "&filter=less:latitude:-0.11");
    }

    #[test]
    fn test_less_than_positive() {
        let fragment = build_fragment("latitude", Less, Some(0.11));
        assert_eq!(fragment, "&filter=less:latitude:0.11");
    }

    #[test]
    fn test_less_than_negative() {
        let fragment = build_fragment("latitude", Less, Some(-0.11));
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
