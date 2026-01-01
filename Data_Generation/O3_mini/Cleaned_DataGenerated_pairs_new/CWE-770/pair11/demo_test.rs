#[cfg(test)]
mod tests {
    use super::Handler;

    #[test]
    fn resource_control_test() {
        let handler = Handler;
        // Using an input that exceeds the safe threshold.
        // The fixed implementation should return an error while the vulnerable version would attempt to spawn too many threads.
        let result = handler.run(1500);
        assert!(result.is_err(), "Expected an error due to too many tasks, but got success.");
    }
}