#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    fn setup_allowed() {
        let _ = fs::create_dir("allowed");
        let safe_file = "allowed/safe.txt";
        let mut file = File::create(safe_file).expect("Unable to create safe file");
        file.write_all(b"Safe Content").expect("Unable to write to safe file");
    }
    #[test]
    fn test_protection() {
        setup_allowed();
        let loader = Loader { base: "allowed/".to_string() };
        let result = loader.get_content("../Cargo.toml");
        assert!(result.is_err(), "Test failed: external file was accessed!");
    }
}
