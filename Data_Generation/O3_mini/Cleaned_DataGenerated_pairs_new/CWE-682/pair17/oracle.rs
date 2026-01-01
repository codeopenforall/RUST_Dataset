#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn calculation_test() {
        let result = run(120, 100);
        assert_eq!(result, 16, "Incorrect percentage calculation, expected 16 but got {}", result);
    }
}
