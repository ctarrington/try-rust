use std::error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
struct Distance {
    value_in_meters: f64,
    unit: DistanceUnit,
}

#[derive(Debug)]
enum DistanceParseError {
    InvalidUnit { raw_unit: String },
    InvalidValue { raw_value: String },
    InvalidFormat,
}

impl Display for DistanceParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            DistanceParseError::InvalidUnit { ref raw_unit } => {
                write!(f, "Invalid unit: {}", raw_unit)
            }
            DistanceParseError::InvalidValue { ref raw_value } => {
                write!(f, "Invalid value: {}", raw_value)
            }
            _ => write!(f, "Invalid format"),
        }
    }
}

impl error::Error for DistanceParseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl TryFrom<String> for Distance {
    type Error = DistanceParseError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = s.split(' ').collect();
        if parts.len() != 2 {
            return Err(DistanceParseError::InvalidFormat);
        }

        let raw_value = parts[0];
        let raw_unit = parts[1];

        let value = match raw_value.parse::<f64>() {
            Ok(value) => value,
            Err(_) => {
                return Err(DistanceParseError::InvalidValue {
                    raw_value: raw_value.to_string(),
                })
            }
        };

        let unit = match raw_unit {
            "m" => DistanceUnit::Meters,
            "mm" => DistanceUnit::Millimeters,
            "km" => DistanceUnit::Kilometers,
            _ => {
                return Err(DistanceParseError::InvalidUnit {
                    raw_unit: raw_unit.to_string(),
                })
            }
        };

        let value_in_meters = match unit {
            DistanceUnit::Meters => value,
            DistanceUnit::Millimeters => value / 1000.0,
            DistanceUnit::Kilometers => value * 1000.0,
        };

        Ok(Distance {
            value_in_meters,
            unit,
        })
    }
}

#[derive(Debug, PartialEq)]
enum DistanceUnit {
    Meters,
    Millimeters,
    Kilometers,
}

impl DistanceUnit {
    fn value(&self, distance: &Distance) -> f64 {
        match self {
            DistanceUnit::Meters => distance.value_in_meters,
            DistanceUnit::Millimeters => distance.value_in_meters * 1000.0,
            DistanceUnit::Kilometers => distance.value_in_meters / 1000.0,
        }
    }
}

#[test]
fn test_distance() {
    let m = Distance {
        value_in_meters: 5.0,
        unit: DistanceUnit::Meters,
    };

    let mm = Distance {
        value_in_meters: 0.005,
        unit: DistanceUnit::Millimeters,
    };

    let km = Distance {
        value_in_meters: 5000.0,
        unit: DistanceUnit::Kilometers,
    };

    assert_eq!(5.0, DistanceUnit::Meters.value(&m));
    assert_eq!(5000.0, DistanceUnit::Millimeters.value(&m));
    assert_eq!(0.005, DistanceUnit::Kilometers.value(&m));

    assert_eq!(5000.0, DistanceUnit::Meters.value(&km));
    assert_eq!(5_000_000.0, DistanceUnit::Millimeters.value(&km));
    assert_eq!(5.0, DistanceUnit::Kilometers.value(&km));

    assert_eq!(0.005, DistanceUnit::Meters.value(&mm));
    assert_eq!(0.000005, DistanceUnit::Kilometers.value(&mm));
    assert_eq!(5.0, DistanceUnit::Millimeters.value(&mm));
}

#[test]
fn test_distance_try_from() {
    let m = Distance::try_from("5 m".to_string()).unwrap();
    assert_eq!(5.0, m.value_in_meters);
    assert_eq!(DistanceUnit::Meters, m.unit);

    let mm = Distance::try_from("5 mm".to_string()).unwrap();
    assert_eq!(0.005, mm.value_in_meters);

    assert!(matches!(
        Distance::try_from("5 cubits".to_string()).unwrap_err(),
        DistanceParseError::InvalidUnit {raw_unit} if raw_unit == "cubits"
    ));

    let invalid_value = Distance::try_from("5.5.5 m".to_string());
    assert!(matches!(
        invalid_value,
        Err(DistanceParseError::InvalidValue { raw_value }) if raw_value == "5.5.5"
    ));

    let invalid_format = Distance::try_from("5".to_string());
    assert!(matches!(
        invalid_format,
        Err(DistanceParseError::InvalidFormat)
    ));
}
