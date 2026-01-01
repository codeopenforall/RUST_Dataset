fn test_oracle() {
    // The test oracle validates that run_app() completes without propagating a panic.
    // In the vulnerable version, run_app() would panic causing this test to fail.
    // In the fixed version, run_app() returns Ok, and the test passes.
    match run_app() {
        Ok(_) => assert!(true, "Application terminated normally"),
        Err(e) => panic!("Test failed: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn verify_run_app() {
        test_oracle();
    }
}