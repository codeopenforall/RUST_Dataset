#[cfg(test)]
mod tests {
    use std::env;
    use std::process::Command;
    use std::path::PathBuf;
    #[test]
    fn test_search_path() {
        let exe_path = PathBuf::from(env!("CARGO_BIN_EXE_testtarget"));
        let output = Command::new(exe_path)
            .env("PATH", "/malicious/path")
            .output()
            .expect("Failed to execute the binary");
        assert!(output.status.success(), "Binary execution failed due to search path vulnerability.");
    }
}
