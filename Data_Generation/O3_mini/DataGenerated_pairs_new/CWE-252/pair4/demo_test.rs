#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    // Oracle function that verifies the correct behavior.
    // It instantiates the resource and calls process with an input longer than allowed.
    // For the vulnerable code, the resource remains unchanged ("init").
    // For the fixed code, the fallback mechanism truncates the input and updates the state to "12345".
    //
    // This test should fail for the vulnerable code and pass for the fixed code.
    #[test]
    fn test_resource_update() {
        // Create resource with initial state "init"
        let resource = Arc::new(Resource {
            data: Mutex::new("init".to_string()),
        });
        // Input exceeding allowed maximum length
        let input = "123456";
        process(Arc::clone(&resource), input);
        let final_state = resource.data.lock().unwrap();
        // In a correct (fixed) version the fallback will update the state to "12345"
        assert_eq!(&*final_state, "12345", "Resource state was not updated correctly");
    }
}