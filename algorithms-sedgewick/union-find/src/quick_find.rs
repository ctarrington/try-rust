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
}

impl<const LENGTH: usize> UnionFind<LENGTH> for QuickFind<LENGTH> {
    fn connected(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }

    fn find(&mut self, site_index: usize) -> usize {
        self.site_to_component.get(site_index)
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

    fn iter(&self) -> impl Iterator<Item = usize> {
        self.site_to_component.iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::QuickFind;
    use crate::union_find::{UnionFind, verify_not_connected};
    use crate::union_find::{connect_evens_odds, orderly_groups, verify_connected};
    use crate::union_find::{single_straight_group, verify_counts};

    #[test]
    fn evens_odds_manual() {
        let mut quick_find: QuickFind<10> = QuickFind::new();
        verify_counts(&quick_find, 0, 10);
        quick_find.union(1, 3);
        verify_counts(&quick_find, 12, 11);
        quick_find.union(3, 5);
        verify_counts(&quick_find, 12 * 2, 13);
        quick_find.union(5, 7);
        verify_counts(&quick_find, 12 * 3, 16);
        quick_find.union(7, 9);
        verify_counts(&quick_find, 12 * 4, 20);

        quick_find.union(0, 2);
        verify_counts(&quick_find, 12 * 5, 21);
        quick_find.union(2, 4);
        verify_counts(&quick_find, 12 * 6, 23);
        quick_find.union(4, 6);
        verify_counts(&quick_find, 12 * 7, 26);
        quick_find.union(6, 8);
        verify_counts(&quick_find, 12 * 8, 30);
        verify_connected(&mut quick_find, 0, 2);
        verify_counts(&quick_find, 12 * 8 + 2, 30);
        verify_not_connected(&mut quick_find, 0, 1);
        verify_counts(&quick_find, 12 * 8 + 4, 30);
    }

    #[test]
    fn evens_odds() {
        let mut quick_find: QuickFind<12> = QuickFind::new();
        verify_counts(&quick_find, 0, 12);

        connect_evens_odds(&mut quick_find);
        verify_counts(&quick_find, 140, 42);

        verify_connected(&mut quick_find, 0, 2);
        verify_not_connected(&mut quick_find, 0, 1);

        let mut quick_find: QuickFind<120> = QuickFind::new();
        connect_evens_odds(&mut quick_find);
        verify_counts(&quick_find, 14396, 3660);
    }

    #[test]
    fn straight_line() {
        let mut quick_find: QuickFind<12> = QuickFind::new();
        single_straight_group(&mut quick_find);
        verify_counts(&quick_find, 154, 78);
        verify_connected(&mut quick_find, 0, 11);

        let mut quick_find: QuickFind<120> = QuickFind::new();
        single_straight_group(&mut quick_find);
        verify_counts(&quick_find, 14518, 7260);
        verify_connected(&mut quick_find, 0, 119);
    }

    #[test]
    fn orderly() {
        let mut quick_find: QuickFind<12> = QuickFind::new();
        orderly_groups(&mut quick_find, 4);
        verify_counts(&quick_find, 112, 24);
        verify_connected(&mut quick_find, 0, 4);

        let mut quick_find: QuickFind<120> = QuickFind::new();
        orderly_groups(&mut quick_find, 4);
        verify_counts(&quick_find, 14152, 1860);
    }
}
