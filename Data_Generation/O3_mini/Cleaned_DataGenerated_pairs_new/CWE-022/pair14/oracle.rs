#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::ErrorKind;
    #[test]
    fn path_traversal_prevention() {
        let base_dir = "./test_temp_data";
        let outside_file = "./test_temp_outside.txt";
        fs::create_dir_all(base_dir).unwrap();
        fs::write(format!("{}/allowed.txt", base_dir), "allowed file").unwrap();
        fs::write(outside_file, "secret data").unwrap();
        let handler = FileHandler::new(base_dir);
        let result = handler.get_content("../test_temp_outside.txt");
        assert!(result.is_err(), "Path traversal was not prevented");
        if let Err(err) = result {
            assert_eq!(err.kind(), ErrorKind::PermissionDenied, "Unexpected error kind");
        }
        fs::remove_dir_all(base_dir).unwrap();
        fs::remove_file(outside_file).unwrap();
    }
}
