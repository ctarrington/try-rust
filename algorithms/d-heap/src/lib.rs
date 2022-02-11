const INITIAL_SIZE: usize = 10;

pub struct Heap {
    values: [Option<i32>; INITIAL_SIZE],
    end_index: usize,
}

impl Heap {
    pub fn new() -> Self {
        const NADA: Option<i32> = None;
        let values = [NADA; INITIAL_SIZE];

        Heap {
            values,
            end_index: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Heap;

    #[test]
    fn construction() {
        let heap = Heap::new();
        assert_eq!(*heap.values.get(0).unwrap(), None);
        assert_eq!(heap.end_index, 0);
    }
}
