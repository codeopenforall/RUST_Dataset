//////////////////////////////////////////
// Test Oracle Function (separate from production code)
//////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn oracle_test() {
        let executor = Executor{};
        let effective = executor.prepare("test_exec");

        // For the fixed version, the effective command path is expected to be absolute.
        // The vulnerable version will simply return "test_exec" which is not absolute.
        assert!(
            Path::new(&effective).is_absolute(),
            "The command path is not absolute; vulnerability exists."
        );
        assert_eq!(
            effective, "/usr/bin/test_exec",
            "The command path is not correctly set to the trusted absolute path."
        );
    }
}