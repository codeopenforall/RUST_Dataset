#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_overflow_detection() {
        let rows = 100_000;
        let cols = 200_000;
        let result = Matrix::create(rows, cols);
        let expected = (rows as u64 * cols as u64) as usize;
        match result {
            Ok(matrix) => {
                assert_eq!(matrix.size(), expected, "Matrix size mismatch due to unchecked overflow");
            },
            Err(err) => {
                assert_eq!(err, "Arithmetic overflow", "Expected arithmetic overflow error");
            }
        }
    }
}
