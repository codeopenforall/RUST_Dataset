#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::fs;

    #[test]
    fn test_error_propagation() {
        // Ensure a clean test environment.
        let _ = fs::remove_file("log.txt");
        let shared = Arc::new(Mutex::new(SharedData::new()));

        // Perform two increments that should succeed.
        shared.increase().expect("First increment should succeed");
        shared.increase().expect("Second increment should succeed");

        // The third increment is expected to fail due to simulated write failure.
        match shared.increase() {
            Ok(_) => panic!("Expected an error on the third increment"),
            Err(e) => assert_eq!(e.to_string(), "Simulated write failure"),
        }
    }
}