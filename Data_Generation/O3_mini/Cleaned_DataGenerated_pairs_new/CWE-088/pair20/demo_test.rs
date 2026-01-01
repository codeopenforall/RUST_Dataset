////////////////////////////////////////////////////////////
// Test oracle for verifying the command injection vulnerability
// This test should fail for the vulnerable implementation and pass for the corrected one.
////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::Executor;

    #[test]
    fn contract_test() {
        // The input simulates an attempt to inject a command.
        let input = "safe; echo injected";
        match Executor::run(input) {
            // For the vulnerable version, the injected command will be executed,
            // resulting in extra output containing "injected", so we expect this branch to be hit.
            Ok(output) => {
                if output.contains("injected") {
                    panic!("Test failed: Command injection succeeded. Output: {}", output);
                } else {
                    // In the vulnerable version, this branch is unlikely.
                    panic!("Test failed: Unexpected output from vulnerable code: {}", output);
                }
            },
            // In the corrected version, the validation logic rejects the input.
            Err(e) => {
                assert_eq!(e, "Invalid characters in input", "Fixed implementation should reject dangerous input");
            }
        }
    }
}