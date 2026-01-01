#[cfg(test)]
mod tests {
    use super::Coordinator;

    #[test]
    fn test_update_trigger() {
        let app = Coordinator::new();
        // For input 42 the expected behavior is to update state to 99 and return Ok.
        let res = app.update_state(42);
        assert!(res.is_ok(), "Expected Ok result for input code 42");
        assert_eq!(app.read_state(), 99, "State should be updated to 99 for input code 42");
    }
}