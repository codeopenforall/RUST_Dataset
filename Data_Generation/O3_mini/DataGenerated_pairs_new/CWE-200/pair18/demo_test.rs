///////////////////////////////
// Test Oracle Functionality //
///////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::panic;

    #[test]
    fn secret_not_leaked() {
        let mydata = Arc::new(Data {
            id: 42,
            secret: "SuperSecretPassword_vulnerable".to_string(),
        });
        let result = panic::catch_unwind(|| {
            process_data(mydata);
        });
        if let Err(e) = result {
            let panic_message = if let Some(s) = e.downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = e.downcast_ref::<String>() {
                s.clone()
            } else {
                "Unknown panic payload".to_string()
            };
            // Assert that the secret is not present in the panic message.
            assert!(
                !panic_message.contains("SuperSecretPassword_vulnerable"),
                "Secret leaked in panic message: {}",
                panic_message
            );
        } else {
            panic!("Expected panic did not occur");
        }
    }
}