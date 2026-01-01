///////////////////////////////////////////////
// Test Oracle Function
///////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn test_out_of_bounds() {
        // For a proper implementation, an index equal to the slice length should return an error.
        //
        // In the unsafe version, run(3) may return a spurious value (or invoke undefined behavior),
        // failing this contract.
        let result = run(3);
        assert!(result.is_err(), "Expected an error for out-of-bounds access (index 3), but got: {:?}", result);
    }
}