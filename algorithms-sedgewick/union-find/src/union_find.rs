use std::fs::File;
use std::io::{Error, Write};
use uuid::Uuid;

// A component is a set of N connected sites
// Those N sites might be points in space, pixels in an image, or computers in a network.
// So we have M components with M << N
// This module provides the trait that all implementations must implement
// as well as common utilities that can work on any implementation
pub trait UnionFind<const LENGTH: usize> {
    // join together p and q
    fn union(&mut self, p: usize, q: usize);

    // answers true if p and q are in the same component
    fn connected(&mut self, p: usize, q: usize) -> bool;

    // answers the component id that the specified site is a member of
    fn find(&mut self, p: usize) -> usize;

    fn count_reads(&self) -> u64;
    fn count_writes(&self) -> u64;

    fn iter(&self) -> impl Iterator<Item = usize>;
}

pub fn connect_evens_odds<const LENGTH: usize, T>(uf: &mut T)
where
    T: UnionFind<LENGTH>,
{
    for index in 0..LENGTH - 2 {
        uf.union(index, index + 2);
    }
}

pub fn orderly_groups<const LENGTH: usize, T>(uf: &mut T, group_count: usize)
where
    T: UnionFind<LENGTH>,
{
    let cycles = LENGTH / group_count;
    for cycle_index in 1..cycles {
        for index in 0..group_count {
            uf.union(
                (cycle_index - 1) * group_count + index,
                cycle_index * group_count + index,
            );
        }
    }
}

pub fn single_straight_group<const LENGTH: usize, T>(uf: &mut T)
where
    T: UnionFind<LENGTH>,
{
    for index in 0..LENGTH - 1 {
        uf.union(index, index + 1);
    }
}

pub fn verify_counts<const LENGTH: usize, T>(uf: &T, reads: u64, writes: u64)
where
    T: UnionFind<LENGTH>,
{
    assert_eq!(reads, uf.count_reads());
    assert_eq!(writes, uf.count_writes());
}

pub fn verify_connected<const LENGTH: usize, T>(uf: &mut T, p: usize, q: usize)
where
    T: UnionFind<LENGTH>,
{
    assert!(uf.connected(p, q));
}

pub fn verify_not_connected<const LENGTH: usize, T>(uf: &mut T, p: usize, q: usize)
where
    T: UnionFind<LENGTH>,
{
    assert!(!uf.connected(p, q));
}

pub fn write_dot<const LENGTH: usize, T>(uf: &mut T, label: &str) -> Result<(), Error>
where
    T: UnionFind<LENGTH>,
{
    let pairs = uf
        .iter()
        .enumerate()
        .map(|(index, component)| format!("{} -> {}", index, component))
        .collect::<Vec<String>>()
        .join("\n");

    let dot = format!(
        "
digraph {{
graph [label=\"Quick Find: {label}\", labelloc=\"t\"];
rankdir=TB
{pairs}
}}
"
    );

    let uuid = Uuid::new_v4();
    let path = format!("./results/quick_find_{}.dot", uuid);
    let mut output = File::create(path)?;
    write!(output, "{}", dot)?;

    Ok(())
}
