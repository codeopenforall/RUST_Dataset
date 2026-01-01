////////////////////////////////////////////////////////////////////////////////////////////////////
// The following test oracle function can be used in a unit testing framework like cargo test.
// It constructs a malicious input consisting of 2000 nested parentheses, with a single 'a' in the middle.
// For the vulnerable version, executing process_input() on this input is expected to trigger a stack 
// overflow (or panic), causing the test to fail. In the fixed version, the recursion depth is limited, 
// and the function returns an error gracefully. An assertion verifies that processing the input does 
// not succeed.
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oracle() {
        // Construct an input with 2000 nested parentheses.
        let depth = 2000;
        let mut input = "(".repeat(depth);
        input.push('a');
        input.push_str(&")".repeat(depth));

        // For the fixed version, process_input should return an error indicating depth exceeded.
        // In the vulnerable version, this input would likely cause a stack overflow before returning.
        let result = process_input(&input);
        assert!(result.is_err(), "Expected parsing to fail due to recursion depth limitation.");
    }
}