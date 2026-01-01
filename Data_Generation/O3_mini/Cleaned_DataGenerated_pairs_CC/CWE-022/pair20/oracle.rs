#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{write, create_dir_all, remove_dir_all};
    use std::path::PathBuf;
    #[test]
    fn test_path_traversal() {
        let base = PathBuf::from("./temp_safe_dir");
        let allowed_file = base.join("allowed.txt");
        let outside_file = base.parent().unwrap().join("outside.txt");
        create_dir_all(&base).unwrap();
        write(&allowed_file, b"allowed content").unwrap();
        write(&outside_file, b"outside content").unwrap();
        let handler = ResourceHandler::new(base.clone());
        let safe_content = handler.load("allowed.txt");
        assert_eq!(safe_content.unwrap(), "allowed content");
        let traversal_path = "../outside.txt";
        let traversal_content = handler.load(traversal_path);
        assert!(
            traversal_content.is_none(),
            "Traversal attack should be blocked, but file was accessed."
        );
        remove_dir_all(&base).unwrap();
        let _ = std::fs::remove_file(outside_file);
    }
}
