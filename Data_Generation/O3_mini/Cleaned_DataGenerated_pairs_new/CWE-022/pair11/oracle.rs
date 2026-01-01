#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::PathBuf;
    #[test]
    fn path_traversal_test() {
        let _ = fs::create_dir_all("./safe_dir");
        let mut safe_file = File::create("./safe_dir/test.txt").unwrap();
        write!(safe_file, "Safe File Content").unwrap();
        let mut secret_file = File::create("./secret.txt").unwrap();
        write!(secret_file, "Secret Data").unwrap();
        let loader = Loader::new(PathBuf::from("./safe_dir"));
        let malicious_input = "../secret.txt";
        let result = loader.load(malicious_input);
        assert!(result.is_err(), "Path traversal vulnerability: Malicious file access was not blocked.");
        let _ = fs::remove_file("./secret.txt");
        let _ = fs::remove_file("./safe_dir/test.txt");
        let _ = fs::remove_dir("./safe_dir");
    }
}
