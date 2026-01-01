#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;
    use std::env;
    use super::Loader;

    // Helper to setup the test environment.
    fn setup_test_env() -> (String, String) {
        // Create a controlled test directory relative to the current working directory.
        let base = "./test_data";
        let secret = "./test_secret.txt";

        // Ensure the base directory exists and create a benign file inside it.
        let _ = fs::create_dir_all(base);
        fs::write(Path::new(base).join("example.txt"), "Example content").unwrap();

        // Create a secret file outside of the base directory.
        fs::write(secret, "Secret content").unwrap();

        (base.to_string(), secret.to_string())
    }

    #[test]
    fn test_path_traversal() {
        let (base, _secret) = setup_test_env();
        let loader = Loader::new(&base);

        // Attempt to load a file using a traversal path.
        let res = loader.load_file("../test_secret.txt");

        // In a secure implementation, the traversal should be detected.
        assert!(res.is_err(), "Path traversal was not properly prevented");

        // Cleanup the test environment.
        let _ = fs::remove_file("./test_secret.txt");
        let _ = fs::remove_dir_all(base);
    }
}