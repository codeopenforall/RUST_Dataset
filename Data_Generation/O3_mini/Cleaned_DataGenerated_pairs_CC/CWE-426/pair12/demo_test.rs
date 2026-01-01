#[cfg(test)]
mod tests {
    use super::Worker;

    #[test]
    fn test_executable_path() {
        let worker = Worker;
        let target = worker.get_target();
        // For the secure version, an absolute path is expected.
        // This test will fail for the vulnerable version (returns a relative path).
        assert!(
            target.starts_with("/"),
            "The executable path must be absolute, but got: {}",
            target
        );
    }
}