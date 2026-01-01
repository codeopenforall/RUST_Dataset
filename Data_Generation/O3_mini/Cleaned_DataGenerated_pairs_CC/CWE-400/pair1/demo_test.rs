/*
The oracle function below serves as the test contract. It should be compiled and run separately
from the main programs. When executed against each version:

- For the vulnerable version:
    * Invoking engine.run_input(200) will cause a panic due to uncontrolled resource consumption.
      The test will catch the panic and then deliberately fail since a graceful error response was expected.
- For the fixed version:
    * Invoking engine.run_input(200) will return an Err("Input load too high"), satisfying the test.
    * Additionally, a low load input (e.g., 10) should result in an Ok(()).

Run the test via your Rust test harness (e.g., using `cargo test`) to verify expected results.
*/

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    #[test]
    fn test_resource_control() {
        let eng = Engine;
        // Test with an overloaded input.
        let res = panic::catch_unwind(|| eng.run_input(200));
        match res {
            Ok(val) => {
                // For the fixed version, the function should return an Err.
                assert!(val.is_err(), "Expected an error for input load exceeding the threshold");
                assert_eq!(val.err().unwrap(), "Input load too high");
            },
            Err(_) => {
                // If the function panics, then it is the vulnerable behavior.
                panic!("Test failed: the function panicked instead of returning an error");
            }
        }

        // Test with a normal input.
        let res_normal = eng.run_input(10);
        assert!(res_normal.is_ok(), "Expected normal processing for input load within the limit");
    }
}