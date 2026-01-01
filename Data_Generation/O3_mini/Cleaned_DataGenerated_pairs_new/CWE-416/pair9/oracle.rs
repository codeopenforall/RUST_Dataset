#[cfg(test)]
mod tests {
    use super::execute;
    #[test]
    fn oracle_test() {
        let result = execute();
        assert_eq!(result, 42, "Expected result to be 42");
    }
}
