#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::fs;
    #[test]
    fn test_error_propagation() {
        let _ = fs::remove_file("log.txt");
        let shared = Arc::new(Mutex::new(SharedData::new()));
        shared.increase().expect("First increment should succeed");
        shared.increase().expect("Second increment should succeed");
        match shared.increase() {
            Ok(_) => panic!("Expected an error on the third increment"),
            Err(e) => assert_eq!(e.to_string(), "Simulated write failure"),
        }
    }
}
