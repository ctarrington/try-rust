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

    pub fn insert(&mut self, value: i32) {
        self.values.push(value);
        self.bubble_up(self.values.len() - 1);
    }

    pub fn top(&mut self) -> Option<i32> {
        if self.values.is_empty() {
            return None;
        }

        let last = self.values.len() - 1;
        self.values.swap(0, last);
        let top_value = self.values.pop();
        self.push_down(0);

        top_value
    }

    fn get_parent_index(&self, index: usize) -> Option<usize> {
        match index {
            0 => None,
            _ => Some((index - 1) / self.child_count),
        }
    }

    fn get_max_child_index(&self, index: usize) -> Option<usize> {
        if self.values.is_empty() || index * self.child_count >= self.values.len() - 1 {
            return None;
        }

        let mut max_child_value = 0;
        let mut max_child_index = 0;
        for offset in 1..=self.child_count {
            let candidate_index = index * self.child_count + offset;
            if candidate_index > self.values.len() - 1 {
                break;
            }

            let candidate_value = *self.values.get(candidate_index).unwrap();
            if candidate_value > max_child_value {
                max_child_index = candidate_index;
                max_child_value = candidate_value;
            }
        }

        Some(max_child_index)
    }

    fn bubble_up(&mut self, index: usize) {
        if let Some(parent_index) = self.get_parent_index(index) {
            if self.values.get(index).unwrap() > self.values.get(parent_index).unwrap() {
                self.values.swap(index, parent_index);
                self.bubble_up(parent_index);
            }
        }
    }

    fn push_down(&mut self, index: usize) {
        if let Some(max_child_index) = self.get_max_child_index(index) {
            self.values.swap(index, max_child_index);
            self.push_down(max_child_index);
        }
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
    fn all_ones_binary() {
        let mut heap = Heap::new(2);

        heap.insert(1);
        heap.insert(1);
        heap.insert(1);

        assert_eq!(vec![1, 1, 1], heap.values);
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

    #[test]
    fn top_binary() {
        let mut heap = Heap::new(2);

        heap.insert(5);
        heap.insert(9);
        heap.insert(3);
        heap.insert(8);

        //     9
        //   8   3
        // 5

        assert_eq!(vec![9, 8, 3, 5], heap.values);
        assert_eq!(Some(9), heap.top());

        //     8
        //   5   3

        assert_eq!(vec![8, 5, 3], heap.values);
        assert_eq!(Some(8), heap.top());
        assert_eq!(Some(5), heap.top());
        assert_eq!(Some(3), heap.top());
        assert_eq!(None, heap.top());
    }
}
