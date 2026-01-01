///////////////////////////
// Test Oracle Function
///////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resource_limit_test() {
        let manager = ResourceManager;
        // Use a triggering input that exceeds safe limits.
        let result = manager.create_resources(200_000);
        // The fixed version should return an error;
        // The vulnerable version returns Ok, which will fail this test.
        match result {
            Ok(_) => panic!("Test failed: Resource allocation did not enforce the limit."),
            Err(msg) => assert_eq!(msg, "Requested resources exceed safe limit"),
        }
    }
}