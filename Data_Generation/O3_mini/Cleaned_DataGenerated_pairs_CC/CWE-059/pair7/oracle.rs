#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::PathBuf;
    fn setup_safe_dir() -> PathBuf {
        let cwd = env::current_dir().unwrap();
        let safe_dir = cwd.join("safe");
        fs::create_dir_all(&safe_dir).unwrap();
        safe_dir
    }
    #[test]
    fn symlink_resolution_test() {
        let safe_dir = setup_safe_dir();
        let legitimate_file = safe_dir.join("test_file.txt");
        {
            let mut file = File::create(&legitimate_file)
                .expect("Failed to create legitimate file");
            writeln!(file, "Safe Content").unwrap();
        }
        let cwd = env::current_dir().unwrap();
        let sensitive_file = cwd.join("sensitive.txt");
        {
            let mut file = File::create(&sensitive_file)
                .expect("Failed to create sensitive file");
            writeln!(file, "Sensitive Data").unwrap();
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::symlink;
            let symlink_path = safe_dir.join("symlink.txt");
            let _ = fs::remove_file(&symlink_path);
            symlink(&sensitive_file, &symlink_path)
                .expect("Failed to create symlink");
            let result = process(symlink_path.to_str().unwrap());
            assert!(
                result.is_err(),
                "Test failure: Symlink resolution vulnerability present"
            );
        }
        #[cfg(windows)]
        {
            unimplemented!();
        }
    }
}
