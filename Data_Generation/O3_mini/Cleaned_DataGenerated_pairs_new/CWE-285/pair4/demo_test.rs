/*
This test oracle is designed to validate the authorization behavior.
It calls the API using a non-privileged credential ("user") and asserts that the access is denied.
For the vulnerable implementation, the test fails because access is incorrectly granted,
while for the corrected version, the test passes as the unauthorized access is properly rejected.
*/

#[cfg(test)]
mod tests {
    use super::process_request;

    #[test]
    fn authorization_test() {
        // Using non-admin credential; expected to be unauthorized.
        let result = process_request("user");
        assert!(result.is_err(), "Unauthorized access should not be allowed");
    }
}