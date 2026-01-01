//////////////////////////////////////////
// Test Oracle Function
//////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::sync::Once;
    
    // This ensures that the test environment is initialized only once.
    static INIT: Once = Once::new();

    // The functions 'execute' and the global state 'GLOBAL' are assumed to be in the parent module.
    #[test]
    fn test_oracle() {
        // Reset the global state before testing.
        unsafe {
            super::GLOBAL = 0;
        }
        let result = super::execute();
        // For the correct (fixed) implementation, the unsafe_update call should fail,
        // and GLOBAL should remain unchanged (i.e., 0).
        // In the vulnerable code, GLOBAL would be incorrectly updated to 1500.
        assert_eq!(result, 0, "Test oracle: expected global state to remain 0 on update failure");
    }
}