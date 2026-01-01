/*
Test Oracle Function:
This test checks that creating a new Processor with a relative executable path causes a panic.
For the fixed (corrected) code, the following unit test will pass.
For the vulnerable code, where no such check exists, the test will fail.

Note: This oracle function should be compiled and run separately from the vulnerable/fixed binaries.
*/

#[cfg(test)]
mod tests {
    use super::Processor;

    #[test]
    #[should_panic(expected = "Only absolute paths are allowed")]
    fn test_absolute_path_enforcement() {
        // This should panic in the corrected version, ensuring that only absolute paths are permitted.
        let _instance = Processor::new("relative_executable");
    }
}