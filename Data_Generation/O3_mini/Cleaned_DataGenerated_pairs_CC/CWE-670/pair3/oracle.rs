#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_security_policy() {
        let mut mgr = UserManager::new();
        mgr.evaluate("guest");
        assert_eq!(mgr.is_allowed(), false, "Guest should not have access");
    }
}
