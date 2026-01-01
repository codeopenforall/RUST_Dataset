#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use std::os::unix::fs::symlink;
    use std::path::PathBuf;
    use std::sync::Arc;
    #[test]
    fn test_symlink_restriction() {
        let base_dir = PathBuf::from("/tmp/test_secure_dir");
        let external_dir = PathBuf::from("/tmp/test_external");
        let _ = fs::create_dir_all(&base_dir);
        let _ = fs::create_dir_all(&external_dir);
        let forbidden_file = external_dir.join("forbidden.txt");
        let mut out_file = File::create(&forbidden_file).expect("Unable to create external file");
        writeln!(out_file, "Sensitive data").expect("Unable to write to external file");
        let symlink_path = base_dir.join("link.txt");
        let _ = fs::remove_file(&symlink_path);
        symlink(&forbidden_file, &symlink_path).expect("Failed to create symlink");
        let file_handler = Arc::new(FileHandler::new(base_dir));
        let result = file_handler.access("link.txt");
        assert!(result.is_err(), "Symlink resolution was not properly restricted");
    }
}
