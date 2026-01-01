#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_buffer_sum() {
        // The expected sum for a sequence of numbers 0 to 9 is 45.
        let result = process();
        assert_eq!(result, 45, "The computed sum should be 45, which will fail if the off-by-one error occurs.");
    }
}