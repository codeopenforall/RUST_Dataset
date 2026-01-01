// This test oracle function is separate from the main application code.
// It asserts that when the "danger" command is processed, an error is returned.
// For the vulnerable code, the test will fail (since it erroneously returns Ok(100)),
// whereas for the fixed code the test passes.
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn oracle() {
        let service = Service::new();
        // When passing a dangerous command, we expect an error.
        // The fixed version returns Err("Danger command not allowed")
        match service.process("danger") {
            Ok(val) => panic!("Test failed: expected error, got Ok({})", val),
            Err(e) => assert_eq!(e, "Danger command not allowed"),
        }
    }
}