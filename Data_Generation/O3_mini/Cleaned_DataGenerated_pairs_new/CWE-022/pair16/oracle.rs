#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;
    #[test]
    fn test_path_traversal_prevention() {
        let base_dir = "./safe_base";
        fs::create_dir_all(base_dir).expect("Failed to create safe directory");
        let inside_file = format!("{}/inside.txt", base_dir);
        fs::write(&inside_file, "This is inside").expect("Failed to write inside file");
        let outside_file = "outside.txt";
        fs::write(outside_file, "This is outside").expect("Failed to write outside file");
        let processor = DirProcessor::new(base_dir);
        let traversal_input = "../";
        let result = processor.process(traversal_input);
        assert!(result.is_err(), "Path traversal was not prevented");
        let _ = fs::remove_file(outside_file);
        let _ = fs::remove_file(&inside_file);
    }
}
