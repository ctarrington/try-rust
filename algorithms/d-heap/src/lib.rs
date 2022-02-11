pub struct Heap {
    values: Vec<i32>,
    child_count: usize,
}

impl Heap {
    pub fn new(child_count: usize) -> Self {
        let values = vec![];

        Heap {
            values,
            child_count,
        }
    }

    fn get_parent_index(&self, index: usize) -> Option<usize> {
        match index {
            0 => None,
            _ => Some((index - 1) / self.child_count),
        }
    }

    fn bubble_up(&mut self, index: usize) {
        let parent_index_option = self.get_parent_index(index);

        if let Some(parent_index) = parent_index_option {
            if self.values.get(index).unwrap() > self.values.get(parent_index).unwrap() {
                self.values.swap(index, parent_index);
                self.bubble_up(parent_index);
            }
        }
    }

    pub fn insert(&mut self, value: i32) {
        self.values.push(value);
        self.bubble_up(self.values.len() - 1);
    }
}

#[cfg(test)]
mod tests {
    use crate::Heap;

    #[test]
    fn insert_binary() {
        let mut heap = Heap::new(2);

        heap.insert(5);
        heap.insert(9);
        heap.insert(3);
        heap.insert(8);

        //     9
        //   8   3
        // 5

        assert_eq!(vec![9, 8, 3, 5], heap.values);
    }

    #[test]
    fn insert_ternary() {
        let mut heap = Heap::new(3);

        heap.insert(5);
        heap.insert(9);
        heap.insert(3);
        heap.insert(8);
        heap.insert(6);

        //         9
        //    6    3     8
        // 5

        assert_eq!(vec![9, 6, 3, 8, 5], heap.values);
    }
}
