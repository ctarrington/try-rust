// A component is a set of N connected sites
// Those N sites might be points in space, pixels in an image, or computers in a network
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
    for cylcle_index in 1..cycles {
        for index in 0..group_count {
            uf.union(
                (cylcle_index - 1) * group_count + index,
                cylcle_index * group_count + index,
            );
        }
    }
}

pub fn verify_counts<const LENGTH: usize, T>(uf: &T, reads: u64, writes: u64)
where
    T: UnionFind<LENGTH>,
{
    assert_eq!(reads, uf.count_reads());
    assert_eq!(writes, uf.count_writes());
}
