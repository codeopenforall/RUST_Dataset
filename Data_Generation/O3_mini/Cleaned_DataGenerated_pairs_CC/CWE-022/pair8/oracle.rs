#[cfg(test)]
mod test_oracle {
    use std::env;
    use std::fs;
    use std::io::ErrorKind;
    use std::path::Path;
    use super::Handler;
    fn run_oracle() {
        let current_dir = env::current_dir().expect("Failed to get current directory");
        let safe_dir = current_dir.join("safe_dir");
        let _ = fs::create_dir_all(&safe_dir);
        let outside_file = current_dir.join("sensitive.txt");
        fs::write(&outside_file, "sensitive content").expect("Failed to write sensitive file");
        let handler = Handler::new(&safe_dir);
        let malicious_input = "../sensitive.txt";
        let result = handler.read_file(malicious_input);
        assert!(
            result.is_err(),
            "Path traversal vulnerability not mitigated: access to sensitive file allowed"
        );
        let _ = fs::remove_file(outside_file);
    }
    #[test]
    fn test_oracle() {
        run_oracle();
    }
}
