//////////////// Test Oracle ////////////////////
// This test oracle function is separate from the main code and is used in unit testing.
// It validates that excessive task allocation is rejected in the fixed version,
// whereas the vulnerable version (lacking the check) would erroneously accept unbounded tasks.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resource_exhaustion_prevention() {
        let processor = Processor;
        // Provide an input that exceeds the recommended maximum.
        let test_tasks = 1500;
        let result = processor.heavy_work(test_tasks);
        // The fixed version returns an error, which is the expected behavior.
        // In the vulnerable version, the error is NOT returned (and threads are spawned),
        // making the test fail.
        assert!(result.is_err(), "Expected error due to resource cap, but got success");
    }
}