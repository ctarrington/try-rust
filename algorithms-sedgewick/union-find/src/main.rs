use crate::quick_find::QuickFind;
use crate::quick_union::QuickUnion;
use crate::union_find::{
    connect_evens_odds, orderly_groups, single_straight_group, verify_connected, verify_counts,
    verify_not_connected, write_dot,
};
use std::io::Error;

mod instrumented_array;
mod quick_find;
mod quick_union;
mod union_find;

fn main() -> Result<(), Error> {
    let mut quick_find: QuickFind<4> = QuickFind::new();
    connect_evens_odds(&mut quick_find);
    verify_connected(&mut quick_find, 0, 2);
    verify_not_connected(&mut quick_find, 1, 2);
    verify_counts(&quick_find, 16, 6);

    let mut quick_find: QuickFind<12> = QuickFind::new();
    orderly_groups(&mut quick_find, 4);
    verify_connected(&mut quick_find, 0, 4);

    write_dot(&mut quick_find, "quick find: even odds")?;

    let mut quick_union: QuickUnion<12> = QuickUnion::new();
    orderly_groups(&mut quick_union, 4);
    verify_connected(&mut quick_union, 0, 4);

    let mut quick_union: QuickUnion<12> = QuickUnion::new();
    single_straight_group(&mut quick_union);
    verify_connected(&mut quick_union, 0, 1);

    write_dot(&mut quick_union, "quick union: even odds")?;

    Ok(())
}
