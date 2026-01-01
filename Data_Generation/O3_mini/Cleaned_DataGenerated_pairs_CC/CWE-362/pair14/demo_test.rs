////////////////////////////////////////////////////////////////////////////////////////////////////
// Test Oracle: Verifying the Invariant that Exactly One Thread Performs the Update
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    // This test verifies that after concurrently invoking process(), the shared counter 
    // remains exactly 1. In the presence of a race condition, the vulnerable version may 
    // result in a counter value greater than 1.
    #[test]
    fn invariant_test() {
        let shared = Arc::new(Shared::new());
        let mut handles = vec![];

        // Increase concurrency by launching multiple threads.
        for _ in 0..10 {
            let s = Arc::clone(&shared);
            handles.push(std::thread::spawn(move || {
                s.process();
            }));
        }
        for h in handles {
            h.join().unwrap();
        }

        // The following assertion should pass for the fixed code and fail for the vulnerable code.
        assert_eq!(shared.value(), 1, "Invariant violated: more than one update occurred");
    }
}