#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn check_subtraction_underflow() {
        let counter = Arc::new(Counter::new(2));
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            counter_clone.subtract(3)
        });
        let result = handle.join().unwrap();
        // For the correct implementation, attempting to subtract more than available
        // should return an error. In the vulnerable variant, the operation would erroneously
        // return a wrapped-around value. This test expects an error.
        match result {
            Ok(val) => panic!("Test failed: Expected an error due to underflow, but got value {}", val),
            Err(e) => assert_eq!(e, "Integer underflow detected"),
        }
    }
}