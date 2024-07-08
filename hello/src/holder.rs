use std::ops::Deref;

#[derive(Debug)]
pub struct Holder {
    pub value: i32,
}

impl Deref for Holder {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[test]
fn test_holder() {
    let holder = Holder { value: 42 };
    assert_eq!(42, holder.value);
    println!("{:?}", holder);
}

#[test]
fn test_deref_holder() {
    let holder = Holder { value: 42 };
    assert_eq!(42, *holder);
}

#[test]
fn test_holders_map_then_filter() {
    let holders = (0..10)
        .map(|i| Holder { value: i })
        .filter(|holder| holder.value % 2 == 0)
        .collect::<Vec<Holder>>();
    assert_eq!(5, holders.len());
    assert_eq!(0, holders[0].value);
    assert_eq!(8, holders[4].value);
    println!("{:?}", holders);
}

#[test]
fn test_holders_mapfilter() {
    let holders = (0..=10)
        .filter_map(|value| {
            if value % 2 == 0 {
                Some(Holder { value })
            } else {
                None
            }
        })
        .collect::<Vec<Holder>>();
    assert_eq!(6, holders.len());
    assert_eq!(0, holders[0].value);
    assert_eq!(10, holders[5].value);
}
