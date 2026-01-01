#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use std::path::PathBuf;
    fn setup_environment() -> (PathBuf, PathBuf) {
        let base_dir = PathBuf::from("data");
        let outside_path = PathBuf::from("secret.txt");
        fs::create_dir_all(&base_dir).unwrap();
        let mut f_inside = fs::File::create(base_dir.join("inside.txt")).unwrap();
        writeln!(f_inside, "valid content").unwrap();
        let mut f_outside = fs::File::create(&outside_path).unwrap();
        writeln!(f_outside, "sensitive content").unwrap();
        (base_dir, outside_path)
    }
    #[test]
    fn test_path_traversal_prevention() {
        let (base_dir, _outside_path) = setup_environment();
        let loader = Loader { base: base_dir };
        let result = loader.load("../secret.txt");
        assert!(result.is_err(), "Path traversal test failed: access outside base directory was allowed");
    }
}
