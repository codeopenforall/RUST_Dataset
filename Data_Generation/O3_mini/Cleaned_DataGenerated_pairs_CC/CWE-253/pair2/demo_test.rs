#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_input() {
        let cmp = Computation;
        // Using a negative input, which should produce a failure.
        let result = cmp.run(-5);
        // The correct interpretation requires false (failure) for negative inputs.
        assert!(!result, "Expected operation failure for negative input, but got success");
    }
}