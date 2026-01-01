/////////////////////////////////////////////////////////////
// Test Oracle Function
/////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    
    #[test]
    fn integrity_check() {
        let sensitive = "SuperSecretPassword1234".to_owned();
        let holder = Arc::new(Holder { secret: sensitive.clone() });
        let res = process_data(holder);
        match res {
            Err(msg) => {
                // The vulnerable version will include the sensitive string in the error message,
                // causing this test to fail. The fixed version will not leak the secret.
                assert!(!msg.contains(&sensitive), "Error message leaked the secret!");
            },
            Ok(_) => panic!("Expected an error due to processing failure"),
        }
    }
}