/*
The following test oracle function invokes the code's run() function and then checks the global free count.
For the fixed implementation, the count is expected to be 1 (one free occurred via manual_release).
In the vulnerable version, the count will be 2 (manual_release and Drop both free the memory),
thereby causing the test to fail.
This test oracle is separate from the implementations.
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_free_protection() {
        // Reset the global counter. In a real test, consider isolation between runs.
        GLOBAL_FREE_COUNT.store(0, Ordering::SeqCst);
        run();
        let count = free_count();
        // In the fixed version, only one free should have occurred.
        assert_eq!(count, 1, "Double free vulnerability detected: free_count = {}", count);
    }
}