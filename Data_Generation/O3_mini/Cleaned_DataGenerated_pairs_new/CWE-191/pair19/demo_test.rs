#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    // This test function invokes the subtraction logic concurrently.
    // For a safe (fixed) implementation, subtracting 10 four times from 5 should saturate at 0.
    // For the vulnerable implementation, unchecked underflow leads to an incorrect (large) value.
    #[test]
    fn test_underflow_prevention() {
        // Create the shared data instance with an initial value that will underflow if unchecked.
        let data = Arc::new(Data::new(5));
        let mut handles = vec![];

        // Spawn four threads each subtracting 10.
        for _ in 0..4 {
            let data_clone = Arc::clone(&data);
            handles.push(thread::spawn(move || {
                data_clone.subtract(10);
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let final_value = data.get();

        // With saturation, the final value must be 0. In the vulnerable version, the value would wrap to
        // a very large number. This assert ensures that the safe implementation passes while the vulnerable one fails.
        assert_eq!(final_value, 0, "Final value did not saturate to 0 as expected");
    }
}