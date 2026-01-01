#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::ErrorKind;
    use std::path::Path;
    fn setup_files() {
        let _ = fs::create_dir("safe_dir");
        fs::write("safe_dir/test.txt", "This is a safe file").unwrap();
        fs::write("secret.txt", "Sensitive secret content").unwrap();
        let link_path = Path::new("safe_dir/link.txt");
        if link_path.exists() {
            let _ = fs::remove_file(link_path);
        }
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink("../secret.txt", "safe_dir/link.txt").unwrap();
        }
        #[cfg(windows)]
        {
            std::os::windows::fs::symlink_file("../secret.txt", "safe_dir/link.txt").unwrap();
        }
    }
    #[test]
    fn test_oracle() {
        setup_files();
        let loader = Loader::new("safe_dir");
        let result = loader.fetch("link.txt");
        assert!(result.is_err(), "Security check failed: symlink allowed access");
        if let Err(e) = result {
            assert_eq!(e.kind(), ErrorKind::PermissionDenied, "Unexpected error kind");
        }
    }
}
