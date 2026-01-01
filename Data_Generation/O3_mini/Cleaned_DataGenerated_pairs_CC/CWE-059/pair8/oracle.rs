#[cfg(test)]
mod tests {
    use std::fs::{self, File};
    use std::io::Write;
    use std::os::unix::fs::symlink;
    use std::path::Path;
    use std::env;
    use std::process::Command;
    use crate::FileResolver;
    #[test]
    fn test_symlink_resolution() {
        let allowed_dir = "/tmp/allowed_test";
        let dangerous_file = "/tmp/dangerous_test.txt";
        let test_file = "test.txt";
        let allowed_path = Path::new(allowed_dir);
        fs::create_dir_all(allowed_path).expect("Failed to create allowed directory");
        let mut dangerous = File::create(dangerous_file).expect("Failed to create dangerous file");
        dangerous.write_all(b"DANGEROUS").expect("Write failed");
        let test_path = allowed_path.join(test_file);
        let _ = fs::remove_file(&test_path);
        symlink(dangerous_file, &test_path).expect("Symlink creation failed");
        let resolver = FileResolver::new(allowed_path.to_path_buf());
        let result = resolver.load(Path::new(test_file));
        match result {
            Ok(content) => {
                panic!("Test failed: expected an error but got content: {}", content);
            }
            Err(e) => {
                assert!(e.contains("Symlink escapes allowed directory"), "Unexpected error: {}", e);
            }
        }
    }
}
