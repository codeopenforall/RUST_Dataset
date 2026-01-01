/*
   The following test oracle function serves as the contract for this pair of implementations.
   It calls the get_item method with an index equal to the length of the buffer.
   - In the secure/fixed version, the function returns an Err("Index out-of-range"), and the test passes.
   - In the insecure/vulnerable version, an out-of-bounds read occurs (or it returns a spurious value), 
     and the test will fail the assertion expecting an error.
     
   This single test demonstrates that the secure version properly handles the boundary condition.
*/
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_oracle() {
        let handler = DataHandler::new(vec![100, 200, 300, 400]);
        let result = handler.get_item(4);
        assert!(
            result.is_err(),
            "Expected error for out-of-bound access, but got: {:?}",
            result
        );
    }
}