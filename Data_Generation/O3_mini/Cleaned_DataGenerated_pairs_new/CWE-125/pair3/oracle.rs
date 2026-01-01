#[cfg(test)]
mod tests {
    use super::DataHolder;
    #[test]
    fn test_out_of_bound_read() {
        let holder = DataHolder::new(vec![10, 20, 30]);
        let result = holder.retrieve(4);
        assert!(result.is_none(), "Expected None for out-of-bound read, but got {:?}", result);
    }
}
