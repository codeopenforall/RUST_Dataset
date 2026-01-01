#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use crate::FileServer;
    #[test]
    fn test_path_traversal_detection() {
        let server = FileServer::new("./data");
        let result = server.process("../outside.txt", "testing data");
        assert!(
            result.is_err(),
            "The file processing did not detect a traversal attempt; vulnerability exists."
        );
    }
}
