#[cfg(test)]
mod tests {
    use super::run_exec;

    // This test oracle simulates the security contract: execution must use an absolute path.
    // The test will fail for the vulnerable version (which uses a relative path) and pass for the fixed version.
    #[test]
    fn test_absolute_path_enforcement() {
        let result = run_exec();
        // Expecting secure execution: the command must start with '/' indicating an absolute path.
        assert!(
            result.is_ok(),
            "The vulnerability was triggered; the command was loaded from an untrusted search path"
        );
        let msg = result.unwrap();
        assert_eq!(
            msg,
            "Command executed securely",
            "The secure execution message was not received"
        );
    }
}