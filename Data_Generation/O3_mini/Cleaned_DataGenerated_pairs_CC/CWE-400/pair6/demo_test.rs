//////////////////////////////////////////
// Test Oracle Function
//
// Description:
// This integration test (designed for use as a unit test) calls the run_simulation()
// function with a specific iteration count (1000 per producer thread). In the vulnerable
// version, the function is expected to panic (or otherwise fail) due to exceeding the
// resource limit, whereas the fixed version should process exactly 4000 messages (4 threads x 1000 iterations).
//
// The test asserts that in the fixed version the function returns Ok(4000),
// and in the vulnerable version it panics or returns an error, causing the test to fail.
//////////////////////////////////////////

#[cfg(test)]
mod tests {
    use std::panic;
    use super::run_simulation;

    #[test]
    fn integration_test() {
         // Catch unwinding to determine if the function panics.
         let result = panic::catch_unwind(|| {
             run_simulation(1000)
         });
         match result {
             Ok(Ok(count)) => {
                 // The fixed version should process exactly 4000 messages.
                 assert_eq!(count, 4000, "Incorrect number of processed messages.");
             }
             Ok(Err(e)) => {
                 panic!("Function returned an error: {}", e);
             }
             Err(_) => {
                 panic!("Function panicked, indicating uncontrolled resource consumption vulnerability.");
             }
         }
    }
}