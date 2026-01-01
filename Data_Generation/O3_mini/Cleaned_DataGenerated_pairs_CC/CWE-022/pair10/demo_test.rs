//////////////////////////////
// Test Oracle Function
//////////////////////////////

// This test oracle verifies that requests attempting path traversal are rejected.
// The test fails for the vulnerable implementation (which erroneously permits traversal)
// and passes for the corrected version.

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_path_traversal_defense() {
        // Set up the file server with the safe directory.
        let server = FileServer::new(PathBuf::from("./safe_dir"));
        
        // Attempt to access a file outside the safe directory.
        // In the fixed version, this should return an error.
        let result = server.serve("../Cargo.toml");
        assert!(result.is_err(), "Traversal attempt was not blocked");
    }
}