#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use std::os::unix::fs::symlink;
    use std::path::PathBuf;
    fn setup_test_env() -> (PathBuf, PathBuf, PathBuf) {
        let safe_dir = PathBuf::from("safe_directory_test");
        let _ = fs::create_dir(&safe_dir);
        let safe_file = safe_dir.join("file.txt");
        let mut f = File::create(&safe_file).unwrap();
        writeln!(f, "Safe Content").unwrap();
        let unsafe_file = PathBuf::from("unsafe.txt");
        let mut uf = File::create(&unsafe_file).unwrap();
        writeln!(uf, "Unsafe Content").unwrap();
        let malicious_link = safe_dir.join("malicious_link");
        let _ = symlink(&unsafe_file, &malicious_link);
        (safe_dir, safe_file, malicious_link)
    }
    #[test]
    fn test_symlink_traversal() {
        let (safe_dir, _safe_file, malicious_link) = setup_test_env();
        let handler = FileHandler::new(safe_dir.to_str().unwrap());
        let res = handler.load_content(malicious_link.to_str().unwrap());
        assert!(res.is_err(), "Access through symlink should be denied in the secure version");
        if let Err(e) = res {
            assert!(e.contains("Access denied"), "Expected an access denial error, got: {}", e);
        }
    }
}
