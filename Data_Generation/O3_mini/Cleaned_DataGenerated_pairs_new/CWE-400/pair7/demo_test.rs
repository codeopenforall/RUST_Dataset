///////////////////////////////////////////////////////////////
// Test Oracle Function
// This test function calls the exported "run" function with a "trigger"
// input (simulating heavy load). It asserts that the total number of tasks
// does not exceed the safe limit (MAX_QUEUE). For the vulnerable version,
// the assertion will fail (total tasks > MAX_QUEUE), while for the fixed version,
// the assertion is satisfied.
///////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::env;
    // Import the run function from the appropriate binary.
    // In practice, this test is compiled separately from the main application.
    use super::run;

    const SAFE_LIMIT: usize = 5000;

    #[test]
    fn test_resource_limit() {
        // Trigger heavy load by requesting 10,000 tasks.
        let total = run(10000);
        // For the fixed version, total tasks must not exceed SAFE_LIMIT.
        // If total > SAFE_LIMIT, the test will fail, exposing the vulnerability.
        assert!(total <= SAFE_LIMIT, "Resource consumption exceeded safe limit: {}", total);
    }
}