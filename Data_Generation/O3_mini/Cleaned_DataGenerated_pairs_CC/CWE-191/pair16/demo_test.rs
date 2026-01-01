///////////////////////////////////////////////////////////////
// Test Oracle Function
// This test function invokes the process() routine with a triggering
// input that causes an integer underflow (attempting to subtract 1
// from an initial value of 0).
//
// The contract is as follows:
// - For the vulnerable version, process() will return Ok(4294967295)
//   due to wrap-around, causing the test to fail because an error was expected.
// - For the fixed version, process() will return Err("Integer underflow detected"),
//   and the test will pass.
///////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_underflow_protection() {
        // Create a Container with initial value 0.
        let container = Container { value: 0 };
        let shared = Arc::new(Mutex::new(container));
        let result = process(shared.clone(), 1);
        // The test expects an error due to underflow protection.
        assert!(result.is_err(), "Underflow was not properly handled");
    }
}