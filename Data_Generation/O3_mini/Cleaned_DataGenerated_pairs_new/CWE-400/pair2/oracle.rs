#[cfg(test)]
mod tests {
    use super::run_app;
    const MAX_ALLOWED: usize = 100000;
    #[test]
    fn regression_test() {
        let count = run_app();
        assert!(count <= MAX_ALLOWED, "Task count {} exceeds allowed maximum {}", count, MAX_ALLOWED);
    }
}
