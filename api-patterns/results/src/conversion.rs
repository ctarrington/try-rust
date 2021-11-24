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

#[derive(Clone, Copy)]
pub enum KitchenError {
    InsufficientIngredients,
    Busy,
    Closed,
    OnFire,
}

pub trait Kitchen {
    fn prepare(&self, thing: &Thing) -> Result<(), KitchenError>;
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
    fn prepare(&self, thing: &Thing) -> Result<(), KitchenError> {
        match &thing.flavor {
            Some(flavor) if self.ingredients.contains(&flavor) => Ok(()),
            _ => Err(KitchenError::InsufficientIngredients),
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
    fn prepare(&self, _thing: &Thing) -> Result<(), KitchenError> {
        Ok(())
    }
}

pub enum FriendlyError {
    SorryCannotPrepare,
    SorryComeBackSoon,
    NeedFlavorDecision,
    NeedSizeDecision,
    NeedSizeAndFlavorDecision,
    NeedMoreCash,
    NeedDifferentPayment,
}

impl From<KitchenError> for FriendlyError {
    fn from(err: KitchenError) -> FriendlyError {
        match err {
            KitchenError::InsufficientIngredients => FriendlyError::SorryCannotPrepare,
            _ => FriendlyError::SorryComeBackSoon,
        }
    }
}

impl From<&str> for FriendlyError {
    fn from(err: &str) -> FriendlyError {
        if err.contains("cash") {
            FriendlyError::NeedMoreCash
        } else {
            FriendlyError::NeedDifferentPayment
        }
    }
}

pub struct Cashier<'a> {
    kitchen: &'a dyn Kitchen,
}

impl<'a> Cashier<'a> {
    pub fn new(kitchen: &'a dyn Kitchen) -> Self {
        Self { kitchen }
    }

    pub fn buy(&self, thing: &Thing, payment: &Payment) -> Result<u32, FriendlyError> {
        Self::validate_order(thing)?;
        self.kitchen.prepare(thing)?;
        let change = Self::process_payment(thing, payment)?;
        Ok(change)
    }

    fn validate_order(thing: &Thing) -> Result<(), FriendlyError> {
        match thing {
            Thing {
                flavor: None,
                size: 0,
            } => Err(FriendlyError::NeedSizeAndFlavorDecision),

            Thing { flavor: _, size: 0 } => Err(FriendlyError::NeedSizeDecision),

            Thing {
                flavor: None,
                size: _,
            } => Err(FriendlyError::NeedFlavorDecision),

            _ => Ok(()),
        }
    }

    // intentionally bad design in using string error messages
    // so the poor from method can clean it up...
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
    use crate::conversion::{
        Cashier, FancyKitchen, Flavor, FriendlyError::*, Kitchen, KitchenError, Payment,
        SimpleKitchen, Thing,
    };

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
        assert!(matches!(response, Err(NeedMoreCash)));

        let response = cashier.buy(
            &Thing {
                size: 22,
                flavor: Some(Flavor::Vanilla),
            },
            &Payment::Card(true),
        );
        assert!(matches!(response, Ok(0)));

        let response = cashier.buy(
            &Thing {
                size: 22,
                flavor: Some(Flavor::Vanilla),
            },
            &Payment::Card(false),
        );
        assert!(matches!(response, Err(NeedDifferentPayment)));

        let response = cashier.buy(
            &Thing {
                size: 22,
                flavor: Some(Flavor::Pistachio),
            },
            &Payment::Cash(100u32),
        );
        assert!(matches!(response, Err(SorryCannotPrepare)));

        let response = cashier.buy(
            &Thing {
                size: 22,
                flavor: None,
            },
            &Payment::Cash(100u32),
        );
        assert!(matches!(response, Err(NeedFlavorDecision)));

        let response = cashier.buy(
            &Thing {
                size: 0,
                flavor: Some(Flavor::Vanilla),
            },
            &Payment::Cash(100u32),
        );
        assert!(matches!(response, Err(NeedSizeDecision)));

        let response = cashier.buy(
            &Thing {
                size: 0,
                flavor: None,
            },
            &Payment::Cash(100u32),
        );
        assert!(matches!(response, Err(NeedSizeAndFlavorDecision)));
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

    #[test]
    fn bad_kitchens() {
        struct MockKitchen {
            error: KitchenError,
        }

        impl Kitchen for MockKitchen {
            fn prepare(&self, _thing: &Thing) -> Result<(), KitchenError> {
                Err(self.error)
            }
        }

        let kitchen = MockKitchen {
            error: KitchenError::OnFire,
        };
        let cashier: Cashier = Cashier::new(&kitchen);

        let response = cashier.buy(
            &Thing {
                size: 40,
                flavor: Some(Flavor::Vanilla),
            },
            &Payment::Cash(100u32),
        );
        assert!(matches!(response, Err(SorryComeBackSoon)));
    }
}
