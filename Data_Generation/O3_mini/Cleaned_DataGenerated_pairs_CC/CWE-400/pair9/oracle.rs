#[cfg(test)]
mod tests {
    use super::run_fn;
    #[test]
    fn resource_consumption_test() {
        let input = vec![1u32; 101];
        let result = run_fn(input);
        assert_eq!(result, 101, "The computed sum does not match the expected value.");
    }
}
