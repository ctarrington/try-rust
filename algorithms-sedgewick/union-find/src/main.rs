fn main() {
    let mut quick_find: QuickFind<2> = QuickFind::new();
    quick_find.union(0, 1);
    assert!(quick_find.connected(0, 1));
}

trait UnionFind<const LENGTH: usize> {
    // join together p and q
    fn union(&mut self, p: usize, q: usize);

    // answers true if p and q are in the same component
    fn connected(&mut self, p: usize, q: usize) -> bool;
}

struct InstrumentedArray<const LENGTH: usize> {
    values: [usize; LENGTH],
    reads: u64,
    writes: u64,
}

impl<const LENGTH: usize> InstrumentedArray<LENGTH> {
    fn new() -> InstrumentedArray<LENGTH> {
        InstrumentedArray {
            values: [0; LENGTH],
            reads: 0,
            writes: 0,
        }
    }

    fn get(&mut self, index: usize) -> usize {
        self.reads += 1;
        self.values[index]
    }

    fn set(&mut self, index: usize, value: usize) {
        self.writes += 1;
        self.values[index] = value;
    }
}

// A component is a set of connected sites
// N sites which might be points in space, or pixels in an image, computers in a network
// M components with M << N where M is the number of connected sets of sites
struct QuickFind<const LENGTH: usize> {
    // index is the site, value is the component
    site_to_component: InstrumentedArray<LENGTH>,
}

impl<const LENGTH: usize> QuickFind<LENGTH> {
    fn new() -> QuickFind<LENGTH> {
        let mut site_to_component = InstrumentedArray::new();
        for index in 0..LENGTH {
            site_to_component.set(index, index);
        }

        QuickFind { site_to_component }
    }

    pub fn find(&mut self, site_index: usize) -> usize {
        self.site_to_component.get(site_index)
    }
}

impl<const LENGTH: usize> UnionFind<LENGTH> for QuickFind<LENGTH> {
    fn connected(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }

    fn union(&mut self, a: usize, b: usize) {
        let component_a = self.find(a);
        let component_b = self.find(b);

        if component_a != component_b {
            for index in 0..LENGTH {
                if self.site_to_component.get(index) == component_a {
                    self.site_to_component.set(index, component_b);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::QuickFind;
    use crate::UnionFind;

    #[test]
    fn evens_odds_manual() {
        let mut quick_find: QuickFind<10> = QuickFind::new();
        assert_eq!(quick_find.site_to_component.reads, 0);
        assert_eq!(quick_find.site_to_component.writes, 10);
        quick_find.union(1, 3);
        assert_eq!(quick_find.site_to_component.reads, 12);
        assert_eq!(quick_find.site_to_component.writes, 11);
        quick_find.union(3, 5);
        assert_eq!(quick_find.site_to_component.reads, 12 * 2);
        assert_eq!(quick_find.site_to_component.writes, 13);
        quick_find.union(5, 7);
        assert_eq!(quick_find.site_to_component.reads, 12 * 3);
        assert_eq!(quick_find.site_to_component.writes, 16);
        quick_find.union(7, 9);
        assert_eq!(quick_find.site_to_component.reads, 12 * 4);
        assert_eq!(quick_find.site_to_component.writes, 20);

        quick_find.union(0, 2);
        assert_eq!(quick_find.site_to_component.reads, 12 * 5);
        assert_eq!(quick_find.site_to_component.writes, 21);
        quick_find.union(2, 4);
        assert_eq!(quick_find.site_to_component.reads, 12 * 6);
        assert_eq!(quick_find.site_to_component.writes, 23);
        quick_find.union(4, 6);
        assert_eq!(quick_find.site_to_component.reads, 12 * 7);
        assert_eq!(quick_find.site_to_component.writes, 26);
        quick_find.union(6, 8);
        assert_eq!(quick_find.site_to_component.reads, 12 * 8);
        assert_eq!(quick_find.site_to_component.writes, 30);

        assert!(quick_find.connected(0, 2));
        assert_eq!(quick_find.site_to_component.reads, 12 * 8 + 2);
        assert!(!quick_find.connected(0, 1));
        assert_eq!(quick_find.site_to_component.reads, 12 * 8 + 4);
    }

    #[test]
    fn evens_odds() {
        let mut quick_find: QuickFind<10> = QuickFind::new();
        connect_evens_odds(&mut quick_find);
        assert!(quick_find.connected(0, 2));
        assert!(!quick_find.connected(0, 1));
    }

    fn connect_evens_odds<const LENGTH: usize, T>(uf: &mut T)
    where
        T: UnionFind<LENGTH>,
    {
        for index in 0..LENGTH - 3 {
            uf.union(index, index + 2);
        }
    }
}
