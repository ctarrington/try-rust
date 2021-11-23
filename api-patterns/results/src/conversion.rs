// mess around with an external api that needs to be converted to internal errors
#[derive(Debug)]
enum Flavor {
    Vanilla,
    Chocolate,
}

#[derive(Debug)]
struct Thing {
    size: i32,
    flavor: Option<Flavor>,
}

#[cfg(test)]
mod tests {
    use crate::conversion::{Flavor, Thing};

    #[test]
    fn simple() {
        let thing = Thing {
            size: 22,
            flavor: Some(Flavor::Chocolate),
        };

        assert_eq!(thing.size, 22);
        assert!(matches!(thing.flavor, Some(Flavor::Chocolate)));
    }
}
