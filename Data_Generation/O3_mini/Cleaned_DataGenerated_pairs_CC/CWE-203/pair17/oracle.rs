#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_error_uniformity() {
        let service = App::new();
        let res_existing = service.verify("alice", "wrong");
        let res_missing = service.verify("bob", "anything");
        match (res_existing, res_missing) {
            (Err(e1), Err(e2)) => {
                assert_eq!(format!("{:?}", e1), format!("{:?}", e2),
                   "Error messages differ, leaking sensitive state.");
            },
            _ => panic!("Expected both cases to fail uniformly"),
        }
    }
}
