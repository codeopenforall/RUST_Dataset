#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_out_of_bounds_update() {
        let result = run();
        assert_eq!(result, Err("index out of bounds"));
    }
}
