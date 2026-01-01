#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;
    use std::env;
    use super::Loader;
    fn setup_test_env() -> (String, String) {
        let base = "./test_data";
        let secret = "./test_secret.txt";
        let _ = fs::create_dir_all(base);
        fs::write(Path::new(base).join("example.txt"), "Example content").unwrap();
        fs::write(secret, "Secret content").unwrap();
        (base.to_string(), secret.to_string())
    }
    #[test]
    fn test_path_traversal() {
        let (base, _secret) = setup_test_env();
        let loader = Loader::new(&base);
        let res = loader.load_file("../test_secret.txt");
        assert!(res.is_err(), "Path traversal was not properly prevented");
        let _ = fs::remove_file("./test_secret.txt");
        let _ = fs::remove_dir_all(base);
    }
}
