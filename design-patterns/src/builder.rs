pub trait Builder<T, E>: Default {
    fn build(self) -> Result<T, E>;
}

pub trait Buildable<T, E, B>
where
    B: Builder<T, E>,
{
    fn builder() -> B;
}

#[macro_export]
macro_rules! with_str {
    ($project:ident, $name:ident, $func:ident) => {
        pub fn $func(mut self, $name: &str) -> Self {
            self.$project.$name = $name.parse().unwrap();
            self
        }
    };
}

#[macro_export]
macro_rules! with_primitive {
    ($project:ident, $name:ident, $func:ident, $type:ty) => {
        pub fn $func(mut self, $name: $type) -> Self {
            self.$project.$name = $name;
            self
        }
    };
}
