use std::fmt::Display;

struct Meters(f64);

struct Millimeters(f64);

#[derive(Clone)]
struct Kilometers(f64);

impl Display for Meters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}m", self.0)
    }
}

impl Display for Millimeters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}mm", self.0)
    }
}

impl Display for Kilometers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}km", self.0)
    }
}

impl From<Meters> for Millimeters {
    fn from(m: Meters) -> Self {
        Millimeters(m.0 * 1000.0)
    }
}

impl From<Meters> for Kilometers {
    fn from(m: Meters) -> Self {
        Kilometers(m.0 / 1000.0)
    }
}

impl From<Millimeters> for Meters {
    fn from(mm: Millimeters) -> Self {
        Meters(mm.0 / 1000.0)
    }
}

impl From<Kilometers> for Meters {
    fn from(km: Kilometers) -> Self {
        Meters(km.0 * 1000.0)
    }
}

impl From<Kilometers> for Millimeters {
    fn from(km: Kilometers) -> Self {
        Millimeters(km.0 * 1_000_000.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn convert_vec(lengths: Vec<Kilometers>) -> Vec<Meters> {
        lengths.into_iter().map(|length| length.into()).collect()
    }

    fn convert_slice(lengths: &[Kilometers]) -> Vec<Meters> {
        lengths
            .into_iter()
            .map(|length| length.clone().into())
            .collect()
    }
    #[test]
    fn test_conversion() {
        let m = Meters(5.0);
        let mm: Millimeters = m.into();
        assert_eq!(5000.0, mm.0);
        assert_eq!("5000mm", format!("{}", mm));

        let m: Meters = mm.into();
        assert_eq!(5.0, m.0);
        assert_eq!("5m", format!("{}", m));

        let m: Meters = Kilometers(1.0).into();
        assert_eq!(1000.0, m.0);

        let mm: Millimeters = Kilometers(1.0).into();
        assert_eq!(1_000_000.0, mm.0);

        let km: Kilometers = Meters(1500.0).into();
        assert_eq!(1.5, km.0);
        assert_eq!("1.5km", format!("{}", km));
    }

    #[test]
    fn test_convert_vec() {
        let kilometers = vec![Kilometers(1.0), Kilometers(2.0)];
        let meters = convert_vec(kilometers);
        assert_eq!(1000.0, meters[0].0);
        assert_eq!(2000.0, meters[1].0);
    }

    #[test]
    fn test_convert_slice_vec() {
        let kilometers = vec![Kilometers(1.0), Kilometers(2.0)];
        let meters = convert_slice(&kilometers);
        assert_eq!(1000.0, meters[0].0);
        assert_eq!(2000.0, meters[1].0);
    }

    #[test]
    fn test_convert_slice_array() {
        let kilometers = &[Kilometers(1.0), Kilometers(2.0)];
        let meters = convert_slice(kilometers);
        assert_eq!(1000.0, meters[0].0);
        assert_eq!(2000.0, meters[1].0);
    }
}
