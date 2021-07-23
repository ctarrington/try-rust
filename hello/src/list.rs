#[derive(Debug)]
pub enum List<T> {
    Empty,
    Node { value: T, next: Box<List<T>> },
}
