/////////////////////////////////////////////////////////////
// The following test oracle function is meant to be used in a
// separate unit test. It calls the common 'run' function and 
// asserts that an out-of-bound update returns an error.
// For the vulnerable code the test will fail (or induce undefined behavior),
// while for the corrected code the test passes reliably.
/////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_out_of_bounds_update() {
        // The expected behavior is to detect an out-of-bounds access and return an error.
        // In the corrected code, run() returns Err("index out of bounds").
        // In the vulnerable code, the unchecked access would succeed (UB) and return Ok(()),
        // causing this test to fail.
        let result = run();
        assert_eq!(result, Err("index out of bounds"));
    }
}