///////////////////////////////////////////////////////////
// Test Oracle: Ensures proper authorization functionality //
///////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oracle() {
        // Create a new controller with "admin" as owner and initial secret 42.
        let ctrl = Controller::new("admin", 42);

        // Attempt modification with an unauthorized caller.
        // For the vulnerable version, this call succeeds and updates the secret to 100,
        // causing the test to fail; for the fixed version, it returns an error.
        let _ = ctrl.modify("intruder", 100);

        // The secret should remain unchanged if the authorization check is correctly enforced.
        let secret = ctrl.read();
        assert_eq!(secret, 42, "Unauthorized update should not change the secret");
    }
}