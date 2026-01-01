#[cfg(test)]
mod tests {
    use super::Computor;
    #[test]
    fn test_oracle() {
        let data = vec![1, 2, 3];
        let comp = Computor::new(data);
        let result = comp.compute();
        assert_eq!(result, 2, "Average calculation is incorrect.");
    }
}
