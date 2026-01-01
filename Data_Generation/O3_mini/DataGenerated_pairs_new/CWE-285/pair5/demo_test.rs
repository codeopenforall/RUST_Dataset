#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    // Import the necessary structures and the function from the module under test.
    // Assume these are available from the library crate produced from the code.
    use super::{User, SensitiveResource, execute};

    #[test]
    fn test_access_control() {
        // Create a sensitive resource instance.
        let resource = Arc::new(Mutex::new(SensitiveResource { secret: String::from("TopSecret") }));
        // Create a non-admin user who is not permitted to retrieve the secret.
        let user = User { name: String::from("test_user"), is_admin: false };
        
        // When a non-admin tries to access the sensitive functionality,
        // the operation must be rejected.
        let result = execute(&user, Arc::clone(&resource));
        assert!(result.is_err(), "Access control test failed: non-admin user should not succeed");
    }
}