use std::fs;
use std::sync::{LazyLock, Mutex};

#[derive(Default)]
// only accessible through the AccessCount to prevent races
struct AccessCountFile {}

impl AccessCountFile {
    fn get_count(&self) -> usize {
        let filename = "./.access_count.txt";
        let file_contents = fs::read_to_string(filename);
        match file_contents {
            Ok(contents) => {
                if contents.is_empty() {
                    fs::write(filename, "0").expect("failed to write to file");
                    0
                } else {
                    let value = contents.parse::<i32>().unwrap();
                    value as usize
                }
            }
            _ => {
                fs::write(filename, "0").expect("failed to write to file");
                0
            }
        }
    }

    fn increment_count(&self) {
        let filename = "./.access_count.txt";
        let value = self.get_count();
        let new_value = value + 1;

        fs::write(filename, format!("{new_value}")).expect("failed to write to file");
    }
}

#[derive(Default)]
/// Thread safe access count keeper
pub struct AccessCount {
    inner: Mutex<AccessCountFile>,
}

impl AccessCount {
    pub fn get_access_count(&self) -> usize {
        self.inner.lock().unwrap().get_count()
    }

    pub fn increment_access_count(&self) {
        self.inner.lock().unwrap().increment_count()
    }
}

pub static ACCESS_COUNT: LazyLock<AccessCount> = LazyLock::new(AccessCount::default);
