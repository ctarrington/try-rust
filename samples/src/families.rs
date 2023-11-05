#[derive(Debug)]
pub struct Person {
    name: String,
    age: u8,
    weight: u8,
    height: u8,
}

impl Person {
    pub fn new(name: String, age: u8, weight: u8, height: u8) -> Self {
        Self {
            name,
            age,
            weight,
            height,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_age(&self) -> u8 {
        self.age
    }

    pub fn get_weight(&self) -> u8 {
        self.weight
    }

    pub fn get_height(&self) -> u8 {
        self.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_person() {
        let joe = Person::new("Joe".parse().unwrap(), 33, 100, 66);
        assert_eq!(joe.name, "Joe");
        assert_eq!(joe.age, 33);
        assert_eq!(joe.weight, 100);
        assert_eq!(joe.height, 66);
        assert_eq!(joe.get_name(), "Joe");
        assert_eq!(joe.get_age(), 33);
        assert_eq!(joe.get_weight(), 100);
        assert_eq!(joe.get_height(), 66);
    }
}
