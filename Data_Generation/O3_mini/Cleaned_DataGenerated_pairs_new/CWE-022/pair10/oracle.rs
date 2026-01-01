#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    #[test]
    fn test_path_traversal_defense() {
        let server = FileServer::new(PathBuf::from("./safe_dir"));
        let result = server.serve("../Cargo.toml");
        assert!(result.is_err(), "Traversal attempt was not blocked");
    }
}
