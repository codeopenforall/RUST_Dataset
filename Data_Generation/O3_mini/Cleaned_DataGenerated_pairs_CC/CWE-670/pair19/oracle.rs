#[cfg(test)]
mod tests {
    use super::execute_tasks;
    #[test]
    fn verify_computation() {
        let mut results = execute_tasks();
        results.sort();
        let mut expected = vec![40, 15, 40, 15];
        expected.sort();
        assert_eq!(results, expected, "The computation did not yield the expected results.");
    }
}
