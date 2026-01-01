#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute() {
        let res = compute();
        assert_eq!(res, 543, "The computed value did not match the expected result.");
    }
}
