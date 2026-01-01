#[cfg(test)]
mod tests {
    use std::env;
    use std::fs;
    use std::io;
    use std::path::PathBuf;
    trait Reader {
        fn read_file(&self, file: &str) -> io::Result<String>;
    }
    use super::FileManager;
    fn setup_allowed_dir() -> PathBuf {
        let base = env::current_dir().unwrap().join("allowed_test");
        let _ = fs::create_dir_all(&base);
        fs::write(base.join("data.txt"), "safe content").unwrap();
        base
    }
    #[test]
    fn test_path_traversal_blocked() {
        let base_dir = setup_allowed_dir();
        let fm = FileManager::new(&base_dir);
        let malicious = "../Cargo.toml";
        let result = fm.read_file(malicious);
        assert!(result.is_err(), "Path traversal bypass detected! Test failed.");
    }
}
