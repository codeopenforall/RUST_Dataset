#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::os::unix::fs::symlink;
    use std::env;
    #[test]
    fn oracle_test() {
        let tmp = env::temp_dir();
        let allowed_dir = tmp.join("oracle_allowed_dir");
        let _ = fs::create_dir_all(&allowed_dir);
        let disallowed_file = tmp.join("oracle_disallowed.txt");
        let _ = fs::remove_file(&disallowed_file);
        let symlink_path = allowed_dir.join("symlink.txt");
        let _ = fs::remove_file(&symlink_path);
        symlink(&disallowed_file, &symlink_path).unwrap();
        let handler = FileHandler { allowed: allowed_dir.clone() };
        let result = handler.write_data(&symlink_path, "TestData");
        if result.is_ok() {
            let content = fs::read_to_string(&disallowed_file).unwrap_or_else(|_| "".into());
            assert_ne!(content, "TestData", "Test failed: vulnerability present - unauthorized modification occurred.");
        } else {
            assert!(true, "Symlink resolution correctly restricted access.");
        }
    }
}
