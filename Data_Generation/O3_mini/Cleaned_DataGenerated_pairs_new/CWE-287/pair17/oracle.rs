#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use super::verify;
    #[test]
    fn test_bad_token() {
        let bad_token = "secXXX"; 
        assert!(!verify(bad_token), "Authentication incorrectly succeeded for invalid token");
    }
}
