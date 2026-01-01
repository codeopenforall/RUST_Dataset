#[cfg(test)]
mod tests {
    use super::DataContainer;
    #[test]
    fn test_out_of_bounds_access() {
        let container = DataContainer::new(10);
        let result = container.read_item(10);
        assert!(result.is_none(), "Expected None for out-of-bound access, but got a value.");
    }
}
