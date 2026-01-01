#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    // This helper function is used in testing both versions.
    // For the fixed version, using a non-admin token should fail authorization.
    // For the vulnerable version, it would incorrectly allow access.
    #[test]
    fn test_authorization() {
        let resource = Arc::new(Resource { secret: 42 });
        unsafe {
            // When using an unauthorized token the call must return an error.
            // This test will fail against the vulnerable implementation and pass for the fixed one.
            let result = resource.get_data("user");
            assert!(
                result.is_err(),
                "Non-admin token should not be authorized to access sensitive data."
            );
        }
    }
}