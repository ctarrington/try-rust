use crate::access_count::ACCESS_COUNT;
use std::string::ToString;
use std::sync::LazyLock;

static CONFIG: LazyLock<String> = LazyLock::new(|| {
    ACCESS_COUNT.increment_access_count();
    println!("loading config");

    "This is the best config".to_string()
});

pub fn get_config() -> &'static String {
    &CONFIG
}

#[cfg(test)]
mod tests {
    use crate::singleton_lazy_lock::{get_config, ACCESS_COUNT};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_get_config_threaded() {
        let original_access_count = ACCESS_COUNT.get_access_count();
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

        let final_access_count = ACCESS_COUNT.get_access_count();
        assert_eq!(final_access_count, original_access_count + 1);
        println!("final_access_count {final_access_count}")
    }
}
