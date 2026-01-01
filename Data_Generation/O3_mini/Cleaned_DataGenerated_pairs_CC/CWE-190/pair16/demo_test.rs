#[cfg(test)]
mod tests {
    use std::process::Command;
    use std::env;

    #[test]
    fn test_oracle() {
        // This test oracle expects the binary to exit with code 0 if the computation is correct.
        // In the vulnerable version the overflow causes an incorrect result and a non-zero exit code.
        // In the fixed version the correct product is computed and the process exits with 0.
        let current_exe = env::args().next().expect("Failed to get current executable");
        let output = Command::new(current_exe)
            .output()
            .expect("Failed to execute process");

        assert_eq!(
            output.status.code(),
            Some(0),
            "Test oracle failed: Unexpected exit code"
        );
    }
}