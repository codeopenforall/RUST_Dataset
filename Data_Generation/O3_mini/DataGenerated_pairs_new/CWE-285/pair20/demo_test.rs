#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_authorization() {
        // Instantiate the manager.
        let mgr = Manager::new();
        // Attempt to run the operation as a non-privileged user.
        let result = mgr.run_operation(Role::User);
        // The correct (fixed) behavior is to deny access.
        assert!(result.is_err(), "Non-privileged user must not be allowed to perform the sensitive operation");
    }
}