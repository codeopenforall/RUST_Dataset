#[cfg(test)]
mod tests {
    use std::fs::{File, OpenOptions};
    use std::io::Write;
    use std::path::Path;
    use std::process::Command;

    // This test oracle uses the process_input function.
    // It intentionally provides a file path outside the allowed directory.
    // In the vulnerable case, the function returns Ok(_) and the test fails,
    // while in the corrected version it returns an error "Insecure file path detected".

    #[test]
    fn test_input_validation() {
        // Prepare a test file outside the safe directory.
        let insecure_path = "/tmp/insecure_test_file";
        {
            let mut test_file = OpenOptions::new()
                .create(true)
                .write(true)
                .open(insecure_path)
                .expect("Failed to create test file");
            writeln!(test_file, "malicious content").unwrap();
        }
        let allowed_directory = Path::new("/safe_dir");

        // Call the function. 
        // For the vulnerable implementation, this will succeed (which is a failure for our test).
        // For the fixed implementation, this should return an Err.
        match super::process_input(insecure_path, allowed_directory) {
            Ok(_) => panic!("Test failed: Insecure file path was accepted"),
            Err(e) => assert_eq!(e, "Insecure file path detected"),
        }

        // Clean up the test file.
        let _ = std::fs::remove_file(insecure_path);
    }
}