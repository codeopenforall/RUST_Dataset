/*
Below is a test oracle function intended for unit testing. It assumes that the core logic of the service (i.e. the 
run_service function) is exposed as part of a library API in both builds (vulnerable and fixed). This single test 
asserts that invoking the service with a "user" role should return no sensitive data.

The test will fail for the vulnerable implementation (which returns Some data for a "user") and pass for the corrected
version (which returns None for a "user").
*/
#[cfg(test)]
mod tests {
    // Assumes run_service is available as part of the crate's public API.
    use crate::run_service;

    #[test]
    fn test_authorization() {
        // "user" role should not receive sensitive data.
        let result = run_service("user");
        assert!(
            result.is_none(),
            "Authorization bypass vulnerability detected: non-admin user gained access."
        );
    }
}