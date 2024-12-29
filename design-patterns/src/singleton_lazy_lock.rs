use std::fs;
use std::string::ToString;
use std::sync::LazyLock;

#[derive(Default)]
struct AccessCountFile {}

impl AccessCountFile {
    fn get_access_count(&self) -> usize {
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

    fn increment_access_count(&self) {
        let filename = "./.access_count.txt";
        let value = self.get_access_count();
        let new_value = value + 1;

        fs::write(filename, format!("{new_value}")).expect("failed to write to file");
    }
}

static ACCESS_COUNT_FILE: LazyLock<AccessCountFile> = LazyLock::new(|| AccessCountFile::default());

static CONFIG: LazyLock<String> = LazyLock::new(|| {
    ACCESS_COUNT_FILE.increment_access_count();
    println!("loading config");

    "This is the best config".to_string()
});

pub fn get_config() -> &'static String {
    &CONFIG
}

#[cfg(test)]
mod tests {
    use crate::singleton_lazy_lock::{get_config, AccessCountFile, ACCESS_COUNT_FILE};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_get_config() {
        let original_access_count = ACCESS_COUNT_FILE.get_access_count();
        assert_eq!(get_config(), &"This is the best config".to_string());
        assert_eq!(get_config(), &"This is the best config".to_string());
        let final_access_count = ACCESS_COUNT_FILE.get_access_count();
        assert_eq!(final_access_count, original_access_count + 1);
    }

    #[test]
    fn test_get_config_threaded() {
        let original_access_count = ACCESS_COUNT_FILE.get_access_count();
        let mut handles = vec![];

        for ctr in 1..5 {
            let handle = thread::spawn(move || {
                let index = ctr;
                println!("run {index}");
                get_config();
                thread::sleep(Duration::from_millis(100));
                println!("done {index}");
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
        println!("all done");

        let final_access_count = ACCESS_COUNT_FILE.get_access_count();
        assert_eq!(final_access_count, original_access_count + 1);
        println!("final_access_count {final_access_count}")
    }
}
