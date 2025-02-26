use std::fmt::Display;

#[derive(Clone, Debug, PartialOrd, PartialEq)]
struct Meters(f64);

#[derive(Clone, Debug, PartialOrd, PartialEq)]
struct Millimeters(Meters);

#[derive(Clone, Debug, PartialOrd, PartialEq)]
struct Kilometers(Meters);

impl Display for Meters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}m", self.0)
    }
}

impl Display for Millimeters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}mm", self.0.clone().0 * 1000.0)
    }
}

impl Display for Kilometers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}km", self.0.clone().0 / 1000.0)
    }
}

impl From<Meters> for Millimeters {
    fn from(m: Meters) -> Self {
        Millimeters(m)
    }
}

impl From<Meters> for Kilometers {
    fn from(m: Meters) -> Self {
        Kilometers(m)
    }
}

impl From<Millimeters> for Meters {
    fn from(mm: Millimeters) -> Self {
        mm.0
    }
}

impl From<Kilometers> for Meters {
    fn from(km: Kilometers) -> Self {
        km.0
    }
}

impl From<Kilometers> for Millimeters {
    fn from(km: Kilometers) -> Self {
        Millimeters(km.0)
    }
}

impl From<f64> for Kilometers {
    fn from(raw: f64) -> Self {
        Kilometers(Meters(raw * 1000.0))
    }
}

impl From<f64> for Millimeters {
    fn from(raw: f64) -> Self {
        Self(Meters(raw / 1000.0))
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
        assert_eq!(Meters(5.0), mm.0);
        assert_eq!("5000mm", format!("{}", mm));

        let m: Meters = mm.into();
        assert_eq!(5.0, m.0);
        assert_eq!("5m", format!("{}", m));

        let m: Meters = Kilometers::from(1.0).into();
        assert_eq!(1000.0, m.0);

        let mm: Millimeters = Kilometers::from(1.2).into();
        assert_eq!(Meters(1_200.0), mm.0);
        assert_eq!("1200000mm", format!("{}", mm));

        let km: Kilometers = Meters(1500.0).into();
        assert_eq!(Meters(1500.0), km.0);
        assert_eq!("1.5km", format!("{}", km));

        let mm: Millimeters = Millimeters::from(1.2);
        assert_eq!(Meters(0.0012), mm.0);
        assert_eq!("1.2mm", format!("{mm}"));
    }

    #[test]
    fn test_convert_vec() {
        let kilometers = vec![Kilometers::from(1.0), Kilometers::from(2.0)];
        let meters = convert_vec(kilometers);
        assert_eq!(1000.0, meters[0].0);
        assert_eq!(2000.0, meters[1].0);
    }

    #[test]
    fn test_convert_slice_vec() {
        let kilometers = vec![Kilometers::from(1.0), Kilometers::from(2.0)];
        let meters = convert_slice(&kilometers);
        assert_eq!(1000.0, meters[0].0);
        assert_eq!(2000.0, meters[1].0);
    }

    #[test]
    fn test_convert_slice_vec_longer() {
        let kilometers = vec![Kilometers::from(1.0), Kilometers::from(2.0)];
        let meters = convert_slice(kilometers.as_slice());
        assert_eq!(1000.0, meters[0].0);
        assert_eq!(2000.0, meters[1].0);
    }

    #[test]
    fn test_convert_slice_array() {
        let kilometers = &[Kilometers::from(1.0), Kilometers::from(2.0)];
        let meters = convert_slice(kilometers);
        assert_eq!(1000.0, meters[0].0);
        assert_eq!(2000.0, meters[1].0);
    }

    #[test]
    fn test_compare_same_units() {
        assert!(Meters(100.0) > Meters(50.0));
        assert!(Kilometers::from(100.0) > Kilometers::from(50.0));
        assert!(Millimeters::from(100.0) > Millimeters::from(50.0));
    }

    #[test]
    fn test_compare_different_units() {
        assert!(Meters(1000.1) > Kilometers::from(1.0).into());
        assert!(Meters::from(Kilometers::from(1.001)) > Millimeters::from(1000.0).into());
    }
}
