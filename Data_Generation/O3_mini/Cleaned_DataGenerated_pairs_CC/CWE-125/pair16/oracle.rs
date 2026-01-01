#[cfg(test)]
mod tests {
    use super::DataStore;
    use std::panic;
    #[test]
    fn test_oracle() {
        let store = DataStore::new();
        let out_of_bounds_index = store.buffer.len();
        let result = panic::catch_unwind(|| {
            store.fetch(out_of_bounds_index)
        });
        assert!(result.is_err(), "Expected panic when accessing an out-of-bounds index");
    }
}
