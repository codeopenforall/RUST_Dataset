#[cfg(test)]
mod tests {
    use super::*;

    // Test oracle: The mitigation is expected to reject relative paths.
    // In the secure (fixed) version, passing a relative path ("vulnerable_app") should yield an error.
    // In the vulnerable version, the relative path might be accepted (or fail with a different error),
    // thereby exposing the vulnerability.
    #[test]
    fn test_untrusted_path() {
        let launcher = Launcher { path: "vulnerable_app".to_owned() };
        let result = launcher.execute();
        match result {
            Err(e) if e.contains("Relative paths are rejected") => {
                // Mitigation is working correctly.
            },
            _ => panic!("Test oracle: untrusted search path vulnerability detected"),
        }
    }
}