#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_authentication() {
        let service = AuthService::new("s3cr3t");
        let result = service.verify("AAAAAA");
        assert!(!result, "Authentication should fail for an invalid token");
    }
}
