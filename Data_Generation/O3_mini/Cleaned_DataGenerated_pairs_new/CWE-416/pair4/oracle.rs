#[cfg(test)]
mod tests {
    use super::compute;
    #[test]
    fn test_compute_result() {
        let res = compute();
        assert_eq!(res, 42, "Test failed: expected 42, got {}", res);
    }
}
