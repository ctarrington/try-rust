// mess around with an external api that needs to be converted to internal errors
// backend errors to front end responses

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum Flavor {
    Vanilla,
    Chocolate,
}

#[derive(Debug)]
pub struct Thing {
    size: u32,
    flavor: Option<Flavor>,
}

#[derive(Debug)]
pub enum Payment {
    Cash(u32),
    Card(bool),
}

pub struct SimpleKitchen {
    ingredients: Vec<Flavor>,
}

impl SimpleKitchen {
    fn new() -> Self {
        Self {
            ingredients: vec![Flavor::Vanilla, Flavor::Chocolate],
        }
    }

    fn prepare(&self, thing: Thing) -> Result<(), &'static str> {
        match thing.flavor {
            Some(flavor) if self.ingredients.contains(&flavor) => Ok(()),
            _ => Err("Sorry we don't have that"),
        }
    }
}

pub struct Cashier<'a> {
    kitchen: &'a SimpleKitchen,
}

impl<'a> Cashier<'a> {
    pub fn new(kitchen: &'a SimpleKitchen) -> Self {
        Self { kitchen: kitchen }
    }

    pub fn buy(&self, thing: Thing, payment: Payment) -> Result<u32, &'static str> {
        let payment_result = match payment {
            Payment::Card(true) => Ok(0),
            Payment::Card(false) => Err("Card declined"),

            Payment::Cash(value) if value >= thing.size => Ok(value - thing.size),
            Payment::Cash(_) => Err("Not enough cash"),
        };

        let kitchen_result = self.kitchen.prepare(thing);

        match kitchen_result {
            Ok(_) => payment_result,
            Err(value) => Err(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::conversion::{Cashier, Flavor, Payment, SimpleKitchen, Thing};

    #[test]
    fn simple() {
        let thing = Thing {
            size: 22,
            flavor: Some(Flavor::Chocolate),
        };

        assert_eq!(thing.size, 22);
        assert!(matches!(thing.flavor, Some(Flavor::Chocolate)));
    }

    #[test]
    fn happy_path() {
        let kitchen = SimpleKitchen::new();
        let cashier: Cashier = Cashier::new(&kitchen);

        let thing = Thing {
            size: 22,
            flavor: Some(Flavor::Vanilla),
        };

        let response = cashier.buy(thing, Payment::Cash(100u32));
        assert!(matches!(response, Ok(78)));
    }
}
