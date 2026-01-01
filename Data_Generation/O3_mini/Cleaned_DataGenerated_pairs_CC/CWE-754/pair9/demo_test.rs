#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cancel_test() {
        // Set the cancellation flag to trigger the exceptional condition.
        let mut flag = true;
        let runner = TaskRunner::new(&mut flag);
        let result = runner.run();
        // The correct behavior (in the fixed version) is to return a cancellation error.
        // The vulnerable version erroneously returns Ok, so this test will fail for it.
        assert_eq!(result, Err(MyError::Cancelled), "Expected cancellation error with flag set to true");
    }
}