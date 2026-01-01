#[cfg(test)]
mod tests {
    use std::env;
    use std::sync::Arc;
    use std::thread;
    use super::Executor;
    #[test]
    fn search_path_oracle() {
        env::set_var("CUSTOM_PATH", "/malicious/path");
        let executor = Executor::new("test run");
        let shared = Arc::new(executor);
        let cloned = Arc::clone(&shared);
        let handler = thread::spawn(move || {
            unsafe { cloned.run_command(&env::var("CUSTOM_PATH").unwrap()) }
        });
        let result = handler.join().unwrap();
        match result {
            Ok(output) => {
                assert!(
                    output.trim() != "test run",
                    "Vulnerability present: malicious path was accepted"
                );
            },
            Err(e) => {
                assert_eq!(e, "Library path not allowed");
            }
        }
    }
}
