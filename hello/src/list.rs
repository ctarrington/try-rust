#[derive(Debug)]
pub enum List {
    Empty,
    Elem(u32, Box<List>),
}
