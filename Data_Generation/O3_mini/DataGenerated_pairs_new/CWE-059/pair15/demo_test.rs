#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::os::unix::fs::symlink;
    use std::path::Path;

    #[test]
    fn test_symlink_traversal_prevention() {
        // Set up a temporary safe directory.
        let base = Path::new("./safe_dir_test");
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(&base).expect("Failed to create safe directory");

        // Create a sensitive file outside the safe directory.
        fs::write("sensitive.txt", "Top Secret Data").expect("Failed to create sensitive file");

        // Within the safe directory, create a symlink that points outside.
        let link_path = base.join("link");
        let _ = fs::remove_file(&link_path);
        symlink("../sensitive.txt", &link_path).expect("Failed to create symlink");

        let mgr = FileManager::new(base);
        let result = mgr.open_file("link");

        // The fixed implementation should block such traversal.
        // For the vulnerable version this test would erroneously succeed.
        assert!(result.is_err(), "Symlink traversal should be blocked.");
    }
}