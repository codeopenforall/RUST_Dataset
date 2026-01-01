#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_saturation() {
        // Here we instantiate the counter with 50 and attempt to subtract 10 ten times.
        // The expected correct behavior is that the counter saturates at 0 instead of wrapping around.
        let counter = Arc::new(Mutex::new(Counter::new(50)));
        run(counter.clone(), 10, 10);
        let final_value = counter.lock().unwrap().get();
        // The test will fail for the vulnerable version (which underflows)
        // and pass for the fixed version (which saturates at 0).
        assert_eq!(final_value, 0, "Counter did not saturate to zero as expected");
    }
}