///////////////////////////////////////////////
// Test Oracle for verifying correct behavior
//
// This test calls the public execute_tasks() helper, sorts the resulting
// vector, and asserts that the fixed logic produces the expected outputs.
// Expected outcomes for tasks with input 20 (High) and 10 (Low) are 40 and 15, respectively,
// repeated twice (due to two threads). The test will fail when run against the vulnerable version.
///////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::execute_tasks;

    #[test]
    fn verify_computation() {
        let mut results = execute_tasks();
        results.sort();
        let mut expected = vec![40, 15, 40, 15];
        expected.sort();
        assert_eq!(results, expected, "The computation did not yield the expected results.");
    }
}