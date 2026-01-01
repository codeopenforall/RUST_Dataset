/*
Test Oracle: This test checks that the process method does not leak the secret.
For the vulnerable implementation the test will fail (as it leaks "SuperSecret!").
For the fixed implementation it will pass.
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secret_leak() {
        let svc = Service::new("admin", "SuperSecret!");
        let result = svc.process();
        match result {
            Err(msg) => {
                // The fixed version should not include the secret "SuperSecret!"
                assert!(!msg.contains("SuperSecret!"), "Sensitive information leaked in message: {}", msg);
            },
            Ok(_) => panic!("Expected an error result, but got Ok."),
        }
    }
}