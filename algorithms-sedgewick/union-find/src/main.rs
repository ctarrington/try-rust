fn main() {
    let mut quick_find: QuickFind<2> = QuickFind::new();
    quick_find.union(0, 1);
    assert!(quick_find.connected(0, 1));
}

// A component is a set of connected sites
// N sites which might be points in space, or pixels in an image, computers in a network
// M components with M << N where M is the number of connected sets of sites
struct QuickFind<const LENGTH: usize> {
    // index is the site, value is the component
    site_to_component: [usize; LENGTH],
}

impl<const LENGTH: usize> QuickFind<LENGTH> {
    fn new() -> QuickFind<LENGTH> {
        let mut site_to_component = [0; LENGTH];
        for (index, item) in site_to_component.iter_mut().enumerate() {
            *item = index;
        }

        QuickFind { site_to_component }
    }

    pub fn find(&self, site_index: usize) -> usize {
        self.site_to_component[site_index]
    }

    pub fn connected(&self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }

    pub fn union(&mut self, a: usize, b: usize) {
        let component_a = self.find(a);
        let component_b = self.find(b);

        if component_a != component_b {
            for component in self.site_to_component.iter_mut() {
                if *component == component_a {
                    *component = component_b;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::QuickFind;

    #[test]
    fn evens_odds() {
        let mut quick_find: QuickFind<10> = QuickFind::new();
        quick_find.union(1, 3);
        quick_find.union(3, 5);
        quick_find.union(5, 7);
        quick_find.union(7, 9);

        quick_find.union(0, 2);
        quick_find.union(2, 4);
        quick_find.union(4, 6);
        quick_find.union(6, 8);

        assert!(quick_find.connected(0, 2));
        assert!(!quick_find.connected(0, 1));
    }
}
