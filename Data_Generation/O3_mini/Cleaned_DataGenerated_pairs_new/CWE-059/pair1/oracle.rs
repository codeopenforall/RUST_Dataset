#[cfg(test)]
mod tests {
    use std::fs::{File, OpenOptions};
    use std::io::Write;
    use std::path::Path;
    use std::process::Command;
    #[test]
    fn test_input_validation() {
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
        match super::process_input(insecure_path, allowed_directory) {
            Ok(_) => panic!("Test failed: Insecure file path was accepted"),
            Err(e) => assert_eq!(e, "Insecure file path detected"),
        }
        let _ = std::fs::remove_file(insecure_path);
    }
}
