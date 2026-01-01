#[cfg(test)]
mod tests {
    use super::Coordinator;
    #[test]
    fn test_update_trigger() {
        let app = Coordinator::new();
        let res = app.update_state(42);
        assert!(res.is_ok(), "Expected Ok result for input code 42");
        assert_eq!(app.read_state(), 99, "State should be updated to 99 for input code 42");
    }
}
