mod families;
use families::Person;

fn main() {
    let joe = Person::new("Joe".parse().unwrap(), 33, 100, 66);
    println!("Hello, {:?}!", joe);
    println!("Hello, {}!", joe.get_name());
    println!("Hello, {}!", joe.get_age());
    println!("Hello, {}!", joe.get_weight());
    println!("Hello, {}!", joe.get_height());
}
