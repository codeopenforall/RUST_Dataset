#[cfg(test)]
mod tests {
    use super::*;

    // This test oracle function ensures that the run method returns an error 
    // message containing the substring "absolute" when the loader attempts to resolve an executable.
    // In the fixed code the error from a missing executable will mention "absolute", 
    // whereas in the vulnerable code it will not. Thus, the test fails for the insecure version.
    #[test]
    fn test_security_check() {
        // Create a loader instance.
        let loader = Loader::new();
        // Invoke the loader with a command name that is expected to be absent.
        let res = loader.run("my_exe", &["--version"]);
        match res {
            Ok(output) => panic!("Unexpected success with output: {}", output),
            Err(err) => {
                // The secure code includes absolute path resolution which results in an error message containing "absolute".
                assert!(err.contains("absolute"),
                    "Security check failed: error message does not indicate absolute path resolution. Received error: {}",
                    err);
            }
        }
    }
}