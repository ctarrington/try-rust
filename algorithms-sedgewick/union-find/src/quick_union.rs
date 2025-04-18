use crate::instrumented_array::InstrumentedArray;
use crate::union_find::UnionFind;

pub struct QuickUnion<const LENGTH: usize> {
    // index is the site, value is the component
    site_to_parent: InstrumentedArray<LENGTH>,
}

impl<const LENGTH: usize> QuickUnion<LENGTH> {
    pub fn new() -> QuickUnion<LENGTH> {
        let mut site_to_parent = InstrumentedArray::new();
        for index in 0..LENGTH {
            site_to_parent.set(index, index);
        }

        QuickUnion { site_to_parent }
    }
}

impl<const LENGTH: usize> UnionFind<LENGTH> for QuickUnion<LENGTH> {
    fn union(&mut self, a: usize, b: usize) {
        let parent_a = self.find(a);
        let parent_b = self.find(b);

        if parent_a != parent_b {
            self.site_to_parent.set(parent_a, parent_b);
        }
    }

    fn connected(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }

    fn find(&mut self, site_index: usize) -> usize {
        let mut index = site_index;
        while index != self.site_to_parent.get(index) {
            index = self.site_to_parent.get(index);
        }
        index
    }

    fn count_reads(&self) -> u64 {
        self.site_to_parent.count_reads()
    }

    fn count_writes(&self) -> u64 {
        self.site_to_parent.count_writes()
    }
}

#[cfg(test)]
mod tests {
    use crate::quick_union::QuickUnion;
    use crate::union_find::{UnionFind, verify_counts};
    use crate::union_find::{connect_evens_odds, orderly_groups};

    #[test]
    fn evens_odds() {
        let mut quick_union: QuickUnion<12> = QuickUnion::new();
        connect_evens_odds(&mut quick_union);
        verify_counts(&quick_union, 20, 22);

        let mut quick_union: QuickUnion<120> = QuickUnion::new();
        connect_evens_odds(&mut quick_union);
        verify_counts(&quick_union, 236, 238);

        assert!(quick_union.connected(0, 2));
        assert!(!quick_union.connected(0, 1));
    }

    #[test]
    fn orderly() {
        let mut quick_union: QuickUnion<12> = QuickUnion::new();
        orderly_groups(&mut quick_union, 4);
        verify_counts(&quick_union, 16, 20);

        let mut quick_union: QuickUnion<120> = QuickUnion::new();
        orderly_groups(&mut quick_union, 40);
        verify_counts(&quick_union, 160, 200);
    }
}
