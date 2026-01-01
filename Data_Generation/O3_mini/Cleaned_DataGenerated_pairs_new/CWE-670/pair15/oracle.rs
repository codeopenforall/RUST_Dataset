#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_access_control() {
        let handler = Handler::new();
        handler.update(150);
        assert!(handler.check(), "Access should be granted for input 150");
    }
}
