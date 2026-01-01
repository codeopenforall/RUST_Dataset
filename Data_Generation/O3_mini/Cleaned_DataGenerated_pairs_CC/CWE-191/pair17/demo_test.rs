#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::thread;
    // Assume that Data is exported appropriately from the module under test.
    use crate::Data;

    #[test]
    fn test_integer_underflow() {
        // Initialize Data with 0 so subtraction should trigger an underflow check.
        let data = Arc::new(Mutex::new(Data::new(0)));
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut d = data_clone.lock().unwrap();
            d.subtract(1)
        });
        let result = handle.join().unwrap();
        match result {
            // For the vulnerable implementation, subtract returns an Ok wrapping value (e.g., 4294967295);
            // for the fixed version, it returns an error.
            Ok(val) => {
                panic!("Test failed: Expected underflow error but got value {}", val);
            }
            Err(e) => {
                assert_eq!(e, "Underflow detected", "Expected exact error message for underflow");
            }
        }
    }
}