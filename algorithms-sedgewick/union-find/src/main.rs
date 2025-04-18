use crate::quick_find::QuickFind;
use crate::quick_union::QuickUnion;
use crate::union_find::{UnionFind, connect_evens_odds, orderly_groups, verify_counts};

mod instrumented_array;
mod quick_find;
mod quick_union;
mod union_find;

fn main() {
    let mut quick_find: QuickFind<4> = QuickFind::new();
    connect_evens_odds(&mut quick_find);
    assert!(quick_find.connected(0, 2));
    assert!(!quick_find.connected(1, 2));
    verify_counts(&quick_find, 16, 6);

    let mut quick_find: QuickFind<12> = QuickFind::new();
    orderly_groups(&mut quick_find, 4);
    assert!(quick_find.connected(0, 4));

    let mut quick_union: QuickUnion<12> = QuickUnion::new();
    orderly_groups(&mut quick_union, 4);
    assert!(quick_union.connected(0, 4));
}
