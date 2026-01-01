////////////////////////////////////////////////////////////
// Test Oracle for Link Following Handling (Unit Test)
// This test should fail when run against the vulnerable implementation 
// (which erroneously accepts symbolic links) and pass using the fixed version.
////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use std::env;
    #[test]
    fn test_symbolic_link_restriction() {
        // Create a temporary directory to hold our test files.
        let tmp_dir = env::temp_dir();
        let real_file_path = tmp_dir.join("real_config.cfg");
        let sym_link_path = tmp_dir.join("symlink_config.cfg");

        // Write a valid configuration to a real file.
        {
            let mut file = File::create(&real_file_path).expect("Failed to create real config");
            writeln!(file, "mode=secure").expect("Failed to write content");
        }
        // Remove any existing symbolic link.
        let _ = fs::remove_file(&sym_link_path);
        // Create a symbolic link pointing to the real file.
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(&real_file_path, &sym_link_path)
                .expect("Failed to create symlink");
        }
        // Instantiate the loader.
        let loader = Loader;
        // Attempt to load configuration via the symbolic link.
        let result = loader.fetch(sym_link_path.to_str().unwrap());
        // In the fixed version, symbolic links are rejected.
        // Therefore, the test expects an error.
        assert!(result.is_err(), "Expected an error when processing a symbolic link");

        // Cleanup test files.
        let _ = fs::remove_file(&real_file_path);
        let _ = fs::remove_file(&sym_link_path);
    }
}