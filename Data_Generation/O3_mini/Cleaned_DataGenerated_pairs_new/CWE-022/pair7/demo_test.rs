#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    // This test attempts to load a file using a malicious path that traverses outside the safe directory.
    // The expected behavior is to return an error indicating a path traversal attempt.
    #[test]
    fn test_malicious_path() {
        let loader = Loader::new("./safe_dir");
        let result = loader.load_file("../secret.txt");
        match result {
            Ok(_) => panic!("Test failed: path traversal was allowed"),
            Err(e) => {
                assert_eq!(
                    e.kind(),
                    io::ErrorKind::PermissionDenied,
                    "Unexpected error kind"
                );
            }
        }
    }
}