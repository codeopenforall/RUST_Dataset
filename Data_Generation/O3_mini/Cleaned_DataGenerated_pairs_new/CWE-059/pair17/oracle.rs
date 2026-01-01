#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use std::env;
    #[test]
    fn test_symbolic_link_restriction() {
        let tmp_dir = env::temp_dir();
        let real_file_path = tmp_dir.join("real_config.cfg");
        let sym_link_path = tmp_dir.join("symlink_config.cfg");
        {
            let mut file = File::create(&real_file_path).expect("Failed to create real config");
            writeln!(file, "mode=secure").expect("Failed to write content");
        }
        let _ = fs::remove_file(&sym_link_path);
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(&real_file_path, &sym_link_path)
                .expect("Failed to create symlink");
        }
        let loader = Loader;
        let result = loader.fetch(sym_link_path.to_str().unwrap());
        assert!(result.is_err(), "Expected an error when processing a symbolic link");
        let _ = fs::remove_file(&real_file_path);
        let _ = fs::remove_file(&sym_link_path);
    }
}
