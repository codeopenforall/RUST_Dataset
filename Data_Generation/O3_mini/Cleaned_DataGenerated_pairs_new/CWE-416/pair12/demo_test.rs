#[cfg(test)]
mod tests {
    use super::*;

    // The test invokes the process function and checks the result.
    // In the unsafe version, memory corruption or undefined behavior should lead to a failing test,
    // while the safe version will return the expected value (100).
    #[test]
    fn memory_safety_test() {
        let result = process();
        assert_eq!(result, 100, "Memory safety violation: Expected 100 as the computed result.");
    }
}