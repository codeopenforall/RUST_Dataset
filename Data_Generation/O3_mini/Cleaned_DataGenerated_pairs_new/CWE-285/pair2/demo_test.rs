#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_authorization() {
        // Create a service instance that requires admin privileges.
        let service = Arc::new(Mutex::new(Service { data: 42, admin_flag: true }));
        // Use a non-admin user.
        let non_admin = User { id: 999, role: "user".to_string() };
        
        // When executing the operation with a non-admin, the secure implementation
        // must return an error; the flawed version will erroneously provide the data.
        let result = execute(Arc::clone(&service), &non_admin);
        assert!(result.is_err(), "Non-admin user should not access sensitive data");
    }
}