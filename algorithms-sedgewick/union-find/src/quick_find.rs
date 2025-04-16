use crate::instrumented_array::InstrumentedArray;
use crate::union_find::UnionFind;

pub struct QuickFind<const LENGTH: usize> {
    // index is the site, value is the component
    site_to_component: InstrumentedArray<LENGTH>,
}

impl<const LENGTH: usize> QuickFind<LENGTH> {
    pub fn new() -> QuickFind<LENGTH> {
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

    fn count_reads(&self) -> u64 {
        self.site_to_component.count_reads()
    }

    fn count_writes(&self) -> u64 {
        self.site_to_component.count_writes()
    }
}

#[cfg(test)]
mod tests {
    use crate::QuickFind;
    use crate::UnionFind;
    use crate::union_find::verify_counts;
    use crate::union_find::{connect_evens_odds, orderly_groups};

    #[test]
    fn evens_odds_manual() {
        let mut quick_find: QuickFind<10> = QuickFind::new();
        assert_eq!(quick_find.site_to_component.count_reads(), 0);
        assert_eq!(quick_find.site_to_component.count_writes(), 10);
        quick_find.union(1, 3);
        assert_eq!(quick_find.site_to_component.count_reads(), 12);
        assert_eq!(quick_find.site_to_component.count_writes(), 11);
        quick_find.union(3, 5);
        assert_eq!(quick_find.site_to_component.count_reads(), 12 * 2);
        assert_eq!(quick_find.site_to_component.count_writes(), 13);
        quick_find.union(5, 7);
        assert_eq!(quick_find.site_to_component.count_reads(), 12 * 3);
        assert_eq!(quick_find.site_to_component.count_writes(), 16);
        quick_find.union(7, 9);
        assert_eq!(quick_find.site_to_component.count_reads(), 12 * 4);
        assert_eq!(quick_find.site_to_component.count_writes(), 20);

        quick_find.union(0, 2);
        assert_eq!(quick_find.site_to_component.count_reads(), 12 * 5);
        assert_eq!(quick_find.site_to_component.count_writes(), 21);
        quick_find.union(2, 4);
        assert_eq!(quick_find.site_to_component.count_reads(), 12 * 6);
        assert_eq!(quick_find.site_to_component.count_writes(), 23);
        quick_find.union(4, 6);
        assert_eq!(quick_find.site_to_component.count_reads(), 12 * 7);
        assert_eq!(quick_find.site_to_component.count_writes(), 26);
        quick_find.union(6, 8);
        assert_eq!(quick_find.site_to_component.count_reads(), 12 * 8);
        assert_eq!(quick_find.site_to_component.count_writes(), 30);

        assert!(quick_find.connected(0, 2));
        assert_eq!(quick_find.site_to_component.count_reads(), 12 * 8 + 2);
        assert!(!quick_find.connected(0, 1));
        assert_eq!(quick_find.site_to_component.count_reads(), 12 * 8 + 4);
    }

    #[test]
    fn evens_odds() {
        let mut quick_find: QuickFind<10> = QuickFind::new();
        assert_eq!(quick_find.count_reads(), 0);
        assert_eq!(quick_find.count_writes(), 10);

        connect_evens_odds(&mut quick_find);
        verify_counts(&quick_find, 12 * 8, 30);

        assert!(quick_find.connected(0, 2));
        assert!(!quick_find.connected(0, 1));
    }

    #[test]
    fn orderly() {
        let mut quick_find: QuickFind<12> = QuickFind::new();
        orderly_groups(&mut quick_find, 4);
        assert!(quick_find.connected(0, 4));
        verify_counts(&quick_find, 114, 24);
    }
}
