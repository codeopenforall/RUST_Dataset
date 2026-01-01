#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let result = process();
        assert_eq!(result, 512, "The result should correctly be 512.");
    }
}
