///////////////////////////////////////////////////////////////
// Oracle Test Function
//
// This test invokes the executable with a malicious input designed to exploit 
// the externally controlled format string vulnerability.
// For the vulnerable code, providing "User: {} extra {}" as an argument will cause a panic 
// (non-zero exit code), while for the corrected code the program runs successfully.
// The test asserts that the fixed version produces a successful (zero exit code) run.
///////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::env;
    use std::process::Command;
    
    #[test]
    fn oracle_test() {
        // Malicious input deliberately contains extra format specifiers.
        let malicious_input = "User: {} extra {}";
        let exe = env::current_exe().expect("Failed to get current executable");
        let output = Command::new(exe)
            .arg(malicious_input)
            .output()
            .expect("Failed to execute process");
        
        // The test expects the process to exit successfully.
        // In the vulnerable variant, the process would panic and exit with a non-zero code.
        assert!(
            output.status.success(),
            "Executable failed with malicious input - vulnerability detected"
        );
    }
}