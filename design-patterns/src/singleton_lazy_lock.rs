use std::fs;
use std::string::ToString;
use std::sync::LazyLock;

fn get_access_count() -> usize {
    let filename = "./.access_count.txt";
    let file_contents = fs::read_to_string(filename);
    match file_contents {
        Ok(contents) => {
            let value = contents.parse::<i32>().unwrap();
            value as usize
        }
        _ => {
            fs::write(filename, "0").expect("failed to write to file");
            0
        }
    }
}

fn increment_access_count() {
    let filename = "./.access_count.txt";
    let value = get_access_count();
    let new_value = value + 1;

    fs::write(filename, format!("{new_value}")).expect("failed to write to file");
}

static CONFIG: LazyLock<String> = LazyLock::new(|| {
    println!("loading config");
    increment_access_count();
    "This is the best config".to_string()
});

pub fn get_config() -> &'static String {
    &*CONFIG
}

#[cfg(test)]
mod tests {
    use crate::singleton_lazy_lock::{get_access_count, get_config};

    #[test]
    fn test_get_config() {
        let original_access_count = get_access_count();
        assert_eq!(get_config(), &"This is the best config".to_string());
        assert_eq!(get_config(), &"This is the best config".to_string());
        let final_access_count = get_access_count();
        assert_eq!(final_access_count, original_access_count + 1);
    }
}
