////////////////////////////////////////////////////////////////////////////////////////////////////
// The following test oracle function validates the security behavior of the code.
// It is expected that when providing a malicious input string that includes shell metacharacters,
// the secure version will reject it (by returning an error) while the vulnerable version will inadvertently
// execute the injected command (resulting in output containing "hacked").
//
// Note: This oracle function should be run separately from both the vulnerable and fixed binaries.
// The test harness executes the applicable binary with the test input and checks the result.
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn injection_oracle() {
        // Malicious input designed to inject an extra command.
        let malicious_input = "malicious; echo hacked";
        let exe = Executor::new(malicious_input.to_string());
        let result = exe.run();
        
        match result {
            // The secure version should detect the forbidden character and return an error.
            Err(err) => {
                assert!(err.to_string().contains("disallowed"), "Fixed code did not block injection");
            },
            // If execution succeeds, ensure that the injected command did not execute.
            Ok(output) => {
                assert!(!output.contains("hacked"), "Vulnerability: injected command executed");
            }
        }
    }
}