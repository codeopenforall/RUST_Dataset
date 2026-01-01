//////////////////////////////////////////////////////////////
// Test Oracle: This function demonstrates that the         //
// vulnerable version returns a specific error message while  //
// the corrected version uses a generic error message.        //
//////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authentication_error_generic() {
        let svc = Service::new();
        svc.register("alice", "secret");
        // Attempt to authenticate with correct username but wrong password.
        let error_msg = svc.authenticate("alice", "wrongpass").unwrap_err();
        // The test expects a generic error message.
        assert_eq!(error_msg, "Invalid credentials", "Oracle: Expected generic error message.");
    }
}