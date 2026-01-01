//////////////////////////////
// Unit Test Oracle Function
//////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    // The test verifies that attempting to subtract a larger value
    // than the current value causes a panic. The corrected version should
    // panic (passing the test), while the vulnerable version would perform
    // an unsafe wrap-around subtraction (failing the test).
    #[test]
    fn underflow_test() {
        let result = panic::catch_unwind(|| {
            run();
        });
        assert!(result.is_err(), "Expected panic due to underflow, but operation completed normally");
    }
}