///////////////////////////////////////////////
// Test Oracle Function
///////////////////////////////////////////////
//
// This function serves as a unit test. It calls the exposed function run_app with a
// dangerous number of jobs that should trigger a resource consumption check.
// In the corrected version, the input (6 jobs) exceeds the allowed limit (5 jobs).
// The test asserts that run_app returns an error for dangerous input.
// For the vulnerable version, run_app returns Ok (thus the test fails).
///////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_resource_limit() {
        // 6 jobs * 10MB = 60MB, which exceeds the safe limit of 50MB.
        let dangerous_jobs = 6;
        let result = run_app(dangerous_jobs);
        // The test expects an error indicating that requested resources are excessive.
        assert!(result.is_err(), "Expected resource limit error, but got success");
    }
}