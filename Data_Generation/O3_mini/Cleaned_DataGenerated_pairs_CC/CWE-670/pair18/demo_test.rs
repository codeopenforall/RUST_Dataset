////////////////////////////
// Test Oracle Function Code
////////////////////////////

#[cfg(test)]
mod tests {
    use super::Executor;

    // The test instantiates an executor in special mode (special == true)
    // and processes a controlled input (4). In the correct implementation,
    // the executor should add 50 to the input, yielding 54.
    // In the vulnerable version, the branch error produces 4 * 2 = 8.
    // This test will fail against the vulnerable code and pass when the issue is fixed.
    #[test]
    fn contract_executor() {
        let mut exec = Executor { special: true, value: 0 };
        let result = exec.process_input(4);
        assert_eq!(
            result, 54,
            "Contract violation: expected result 54, got {}",
            result
        );
    }
}