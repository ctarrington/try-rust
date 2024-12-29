use std::fs;
use std::sync::{LazyLock, Mutex};
use uuid::Uuid;

// only accessible through the AccessCount to prevent races
struct AccessCountFile {
    filename: String,
}

impl AccessCountFile {
    fn get_count(&self) -> usize {
        let file_contents = fs::read_to_string(self.filename.clone());
        match file_contents {
            Ok(contents) => {
                if contents.is_empty() {
                    fs::write(self.filename.clone(), "0").expect("failed to write to file");
                    0
                } else {
                    let value = contents.parse::<i32>().unwrap();
                    value as usize
                }
            }
            _ => {
                fs::write(self.filename.clone(), "0").expect("failed to write to file");
                0
            }
        }
    }

    fn increment_count(&self) {
        let value = self.get_count();
        let new_value = value + 1;

        fs::write(self.filename.clone(), format!("{new_value}")).expect("failed to write to file");
    }
}

impl Default for AccessCountFile {
    fn default() -> Self {
        let postfix = Uuid::new_v4();
        let postfix = String::from(postfix);
        let filename = format!(".access_count{postfix}.txt");
        AccessCountFile { filename }
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

#[cfg(test)]
mod tests {
    use crate::access_count::AccessCount;
    use std::sync::LazyLock;
    use std::thread;
    use std::time::Duration;

    // mirror the pub ACCESS_COUNT for isolation
    static ACCESS_COUNT: LazyLock<AccessCount> = LazyLock::new(AccessCount::default);

    #[test]
    fn concurrent_increment() {
        let original_access_count = ACCESS_COUNT.get_access_count();
        let mut handles = vec![];

        let thread_count = 10;
        for ctr in 1..=thread_count {
            let handle = thread::spawn(move || {
                let index = ctr;
                println!("run {index}");
                ACCESS_COUNT.increment_access_count();
                thread::sleep(Duration::from_millis(100));
                println!("done {index}");
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
        println!("all done");

        let final_access_count = ACCESS_COUNT.get_access_count();
        assert_eq!(final_access_count, original_access_count + thread_count);
    }
}
