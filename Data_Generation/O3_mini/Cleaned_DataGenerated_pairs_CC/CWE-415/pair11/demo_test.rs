/*
Test Oracle:
This unit test function is designed to run the 'process' function (common to both modules).
It resets the global counter and asserts that the process function returns Ok.
For the vulnerable version (double free), the condition GLOBAL_COUNT == 1 is violated,
and the test will fail. For the fixed version, the test passes.
*/
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::Ordering;

    #[test]
    fn test_resource_handling() {
        // Reset the global counter prior to running the test.
        GLOBAL_COUNT.store(0, Ordering::SeqCst);
        let result = process();
        assert!(result.is_ok(), "Resource handling failed due to double free vulnerability");
    }
}