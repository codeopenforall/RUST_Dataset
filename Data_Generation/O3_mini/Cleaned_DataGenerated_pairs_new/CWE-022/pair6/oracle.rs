#[cfg(test)]
mod tests {
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::PathBuf;
    use super::FileManager;
    #[test]
    fn test_path_traversal_defense() {
        let base = PathBuf::from("./data");
        fs::create_dir_all(&base).unwrap();
        let secret_path = PathBuf::from("./secret.txt");
        let mut secret_file = File::create(&secret_path).expect("Failed to create secret file");
        writeln!(secret_file, "This is secret!").expect("Failed to write secret data");
        let fm = FileManager::new(base);
        let result = fm.open_file("../secret.txt");
        assert!(result.is_err(), "Traversal attack was not prevented");
        fs::remove_file(&secret_path).unwrap();
    }
}
