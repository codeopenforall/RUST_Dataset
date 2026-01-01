#[cfg(test)]
mod tests {
    use super::process;
    #[test]
    fn test_buffer_sum() {
        let result = process();
        assert_eq!(result, 45, "The computed sum should be 45, which will fail if the off-by-one error occurs.");
    }
}
