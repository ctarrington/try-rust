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
    InvalidFormat { raw: String },
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
            DistanceParseError::InvalidFormat { ref raw } => {
                write!(f, "Invalid format: {}", raw)
            }
        }
    }
}

impl error::Error for DistanceParseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

fn split_distance(raw: &str) -> Result<(&str, &str), DistanceParseError> {
    let parts: Vec<&str> = raw.trim().split(' ').collect();
    if parts.len() != 2 {
        return Err(DistanceParseError::InvalidFormat {
            raw: raw.to_string(),
        });
    }

    Ok((parts[0], parts[1]))
}

fn parse_value(raw: &str) -> Result<f64, DistanceParseError> {
    match raw.parse::<f64>() {
        Ok(value) => Ok(value),
        Err(_) => Err(DistanceParseError::InvalidValue {
            raw_value: raw.to_string(),
        }),
    }
}

impl TryFrom<String> for Distance {
    type Error = DistanceParseError;

    fn try_from(raw: String) -> Result<Self, Self::Error> {
        let (raw_value, raw_unit) = split_distance(&raw)?;
        let value = parse_value(raw_value)?;
        let unit: DistanceUnit = raw_unit.to_string().try_into()?;
        Ok(unit.distance(value))
    }
}

#[derive(Debug, PartialEq)]
enum DistanceUnit {
    Meters,
    Centimeters,
    Millimeters,
    Kilometers,
}

impl TryFrom<String> for DistanceUnit {
    type Error = DistanceParseError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "km" => Ok(DistanceUnit::Kilometers),
            "m" => Ok(DistanceUnit::Meters),
            "cm" => Ok(DistanceUnit::Centimeters),
            "mm" => Ok(DistanceUnit::Millimeters),
            _ => Err(DistanceParseError::InvalidUnit { raw_unit: s }),
        }
    }
}

impl DistanceUnit {
    fn conversion_factor(&self) -> f64 {
        match self {
            DistanceUnit::Meters => 1.0,
            DistanceUnit::Centimeters => 100.0,
            DistanceUnit::Millimeters => 1000.0,
            DistanceUnit::Kilometers => 0.001,
        }
    }

    fn value(&self, distance: &Distance) -> f64 {
        distance.value_in_meters * self.conversion_factor()
    }

    fn distance(self, value: f64) -> Distance {
        Distance {
            value_in_meters: value / self.conversion_factor(),
            unit: self,
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

    let cm = DistanceUnit::Centimeters.distance(5.0);
    assert_eq!(0.05, cm.value_in_meters);
}

#[test]
fn test_distance_try_from() {
    let m = Distance::try_from("5 m".to_string()).unwrap();
    assert_eq!(5.0, m.value_in_meters);
    assert_eq!(DistanceUnit::Meters, m.unit);

    let mm = Distance::try_from("5 mm ".to_string()).unwrap();
    assert_eq!(0.005, mm.value_in_meters);

    let cm = Distance::try_from("5 cm".to_string()).unwrap();
    assert_eq!(0.05, cm.value_in_meters);

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
        Err(DistanceParseError::InvalidFormat { raw }) if raw == "5"
    ));

    let invalid_format = Distance::try_from("".to_string());
    assert!(matches!(
        invalid_format,
        Err(DistanceParseError::InvalidFormat { raw }) if raw == ""
    ));
}

#[test]
fn test_format_error() {
    let invalid_unit = Distance::try_from("5 cubits".to_string()).unwrap_err();
    assert_eq!("Invalid unit: cubits", invalid_unit.to_string());

    let invalid_value = Distance::try_from("5.5.5 m".to_string()).unwrap_err();
    assert_eq!("Invalid value: 5.5.5", invalid_value.to_string());

    let invalid_format = Distance::try_from("5".to_string()).unwrap_err();
    assert_eq!("Invalid format: 5", invalid_format.to_string());
}
