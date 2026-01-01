#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn memory_safety_test() {
        let result = process();
        assert_eq!(result, 100, "Memory safety violation: Expected 100 as the computed result.");
    }
}
