//////////////////////////////////////////////////////////////
// Test Oracle: This test verifies that the fixed version prevents path
// traversal, while the vulnerable version (if tested independently) would
// allow access to directories outside the intended safe directory.
// 
// The test sets up a safe directory with an inside file and an outside file.
// It then attempts to use a directory traversal input ("../") to access the 
// outside file. The expected behavior for the secure implementation is to return
// an error.
//////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_path_traversal_prevention() {
        let base_dir = "./safe_base";
        // Ensure the safe directory exists.
        fs::create_dir_all(base_dir).expect("Failed to create safe directory");
        // Create a file inside the safe directory.
        let inside_file = format!("{}/inside.txt", base_dir);
        fs::write(&inside_file, "This is inside").expect("Failed to write inside file");
        // Create a file outside the safe directory.
        let outside_file = "outside.txt";
        fs::write(outside_file, "This is outside").expect("Failed to write outside file");

        let processor = DirProcessor::new(base_dir);
        // Using a traversal input that should be rejected in the fixed implementation.
        let traversal_input = "../";
        let result = processor.process(traversal_input);
        // The secure code should not allow path traversal and return an error.
        assert!(result.is_err(), "Path traversal was not prevented");

        // Cleanup
        let _ = fs::remove_file(outside_file);
        let _ = fs::remove_file(&inside_file);
    }
}