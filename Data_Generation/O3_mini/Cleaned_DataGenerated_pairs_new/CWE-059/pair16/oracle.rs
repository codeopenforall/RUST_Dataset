#[cfg(test)]
mod tests {
    use std::fs;
    use std::os::unix::fs::symlink;
    use std::path::Path;
    use crate::Manager;
    #[test]
    fn path_validation_oracle() {
        let safe_dir = "safe_dir";
        let _ = fs::create_dir_all(safe_dir);
        let target_file = "unsafe_target.txt";
        fs::write(target_file, "sensitive information").expect("Unable to write target file");
        let symlink_path = format!("{}/link.txt", safe_dir);
        let _ = fs::remove_file(&symlink_path);
        symlink(Path::new("../unsafe_target.txt"), &symlink_path)
            .expect("Failed to create symbolic link");
        let mgr = Manager::new(safe_dir);
        let res = mgr.process("link.txt");
        assert!(
            res.is_err(),
            "Test failed: Symbolic link resolution allowed access to an unintended file."
        );
    }
}
