// common utilities that can work on any implementation of union-find
pub trait UnionFind<const LENGTH: usize> {
    // join together p and q
    fn union(&mut self, p: usize, q: usize);

    // answers true if p and q are in the same component
    fn connected(&mut self, p: usize, q: usize) -> bool;

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
