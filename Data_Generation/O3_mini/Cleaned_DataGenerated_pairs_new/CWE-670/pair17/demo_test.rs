#[cfg(test)]
mod tests {
    use super::run_app;
    
    #[test]
    fn control_flow_integrity_test() {
        // For approved computation, input 5 should yield 10.
        // In the vulnerable version the unsafely updated branch would yield 50.
        // This test should fail on the vulnerable version and pass on the fixed version.
        let res = run_app();
        assert_eq!(res, 10, "Control flow hijacking detected: computed value is not as expected");
    }
}