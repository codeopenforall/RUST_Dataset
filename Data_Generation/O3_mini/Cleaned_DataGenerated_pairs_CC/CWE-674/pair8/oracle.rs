#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn oracle() {
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
