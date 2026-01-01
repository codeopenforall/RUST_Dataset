//////////////////////////////
// Test Oracle Function
//////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access_control() {
        // Create a service instance with sensitive data.
        let service = Service::new("very_sensitive_info");
        // Create a non-privileged user.
        let non_admin = User { name: "TestUser".to_string(), is_admin: false };
        // Invoke the access control method.
        let result = service.access_data(&non_admin);
        // The fixed code should return an error for non-admin users.
        // In the vulnerable implementation, the result would be Ok(secret), causing this test to fail.
        assert!(result.is_err(), "Non-admin user should not be allowed access to secure data");
    }
}