/*
The test oracle below verifies the correct behavior for a timeout scenario.
When process_task(false) is called, the proper handling code should detect the timeout and return an Err with the message
"operation timed out". In the vulnerable version, the function incorrectly returns Ok(1000), causing the test to fail.
*/
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn oracle_test() {
         // Trigger the timeout scenario by passing 'false'
         let res = process_task(false);
         // Expect an error for the fixed code when a timeout occurs.
         assert!(res.is_err(), "Expected an error on timeout condition.");
         assert_eq!(res.unwrap_err(), "operation timed out", "Error message mismatch.");
    }
}