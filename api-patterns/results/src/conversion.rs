// mess around with an external api that needs to be converted to internal errors
// backend errors to front end responses

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum Flavor {
    Vanilla,
    Chocolate,
    Pistachio,
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

pub trait Kitchen {
    fn prepare(&self, thing: &Thing) -> Result<(), &'static str>;
}

pub struct SimpleKitchen {
    ingredients: Vec<Flavor>,
}

impl SimpleKitchen {
    #[allow(dead_code)]
    fn new() -> Self {
        Self {
            ingredients: vec![Flavor::Vanilla, Flavor::Chocolate],
        }
    }
}

impl Kitchen for SimpleKitchen {
    fn prepare(&self, thing: &Thing) -> Result<(), &'static str> {
        match &thing.flavor {
            Some(flavor) if self.ingredients.contains(&flavor) => Ok(()),
            _ => Err("Sorry we don't have that"),
        }
    }
}

pub struct FancyKitchen {}

impl FancyKitchen {
    #[allow(dead_code)]
    fn new() -> Self {
        Self {}
    }
}

impl Kitchen for FancyKitchen {
    fn prepare(&self, _thing: &Thing) -> Result<(), &'static str> {
        Ok(())
    }
}

pub struct Cashier<'a> {
    kitchen: &'a dyn Kitchen,
}

impl<'a> Cashier<'a> {
    pub fn new(kitchen: &'a dyn Kitchen) -> Self {
        Self { kitchen }
    }

    pub fn buy(&self, thing: &Thing, payment: &Payment) -> Result<u32, &'static str> {
        Self::validate_order(thing)?;
        self.kitchen.prepare(thing)?;
        Self::process_payment(thing, payment)
    }

    fn validate_order(thing: &Thing) -> Result<(), &'static str> {
        match thing {
            Thing {
                flavor: None,
                size: 0,
            } => Err("You need to tell me what size and flavor you want!"),

            Thing { flavor: _, size: 0 } => Err("You need to tell me what size you want!"),

            Thing {
                flavor: None,
                size: _,
            } => Err("You need to tell me what flavor you want!"),

            _ => Ok(()),
        }
    }

    fn process_payment(thing: &Thing, payment: &Payment) -> Result<u32, &'static str> {
        match payment {
            Payment::Card(true) => Ok(0),
            Payment::Card(false) => Err("Card declined"),

            Payment::Cash(value) if value >= &thing.size => Ok(value - thing.size),
            Payment::Cash(_) => Err("Not enough cash"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::conversion::{Cashier, FancyKitchen, Flavor, Payment, SimpleKitchen, Thing};

    #[test]
    fn simple_path() {
        let kitchen = SimpleKitchen::new();
        let cashier: Cashier = Cashier::new(&kitchen);

        let response = cashier.buy(
            &Thing {
                size: 22,
                flavor: Some(Flavor::Vanilla),
            },
            &Payment::Cash(100u32),
        );
        assert!(matches!(response, Ok(78)));

        let response = cashier.buy(
            &Thing {
                size: 22,
                flavor: Some(Flavor::Vanilla),
            },
            &Payment::Cash(20u32),
        );
        assert!(matches!(response, Err("Not enough cash")));

        let response = cashier.buy(
            &Thing {
                size: 22,
                flavor: Some(Flavor::Pistachio),
            },
            &Payment::Cash(100u32),
        );
        assert!(matches!(response, Err("Sorry we don't have that")));

        let response = cashier.buy(
            &Thing {
                size: 22,
                flavor: None,
            },
            &Payment::Cash(100u32),
        );
        assert!(matches!(
            response,
            Err("You need to tell me what flavor you want!")
        ));

        let response = cashier.buy(
            &Thing {
                size: 0,
                flavor: Some(Flavor::Vanilla),
            },
            &Payment::Cash(100u32),
        );
        assert!(matches!(
            response,
            Err("You need to tell me what size you want!")
        ));

        let response = cashier.buy(
            &Thing {
                size: 0,
                flavor: None,
            },
            &Payment::Cash(100u32),
        );
        assert!(matches!(
            response,
            Err("You need to tell me what size and flavor you want!")
        ));
    }

    #[test]
    fn fancy_path() {
        let kitchen = FancyKitchen::new();
        let cashier: Cashier = Cashier::new(&kitchen);

        let response = cashier.buy(
            &Thing {
                size: 40,
                flavor: Some(Flavor::Pistachio),
            },
            &Payment::Cash(100u32),
        );
        assert!(matches!(response, Ok(60)));
    }
}
