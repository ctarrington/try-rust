pub struct InstrumentedArray<const LENGTH: usize> {
    values: [usize; LENGTH],
    reads: u64,
    writes: u64,
}

impl<const LENGTH: usize> InstrumentedArray<LENGTH> {
    pub fn new() -> InstrumentedArray<LENGTH> {
        InstrumentedArray {
            values: [0; LENGTH],
            reads: 0,
            writes: 0,
        }
    }

    pub fn get(&mut self, index: usize) -> usize {
        self.reads += 1;
        self.values[index]
    }

    pub fn set(&mut self, index: usize, value: usize) {
        self.writes += 1;
        self.values[index] = value;
    }

    pub fn count_reads(&self) -> u64 {
        self.reads
    }

    pub fn count_writes(&self) -> u64 {
        self.writes
    }
}
