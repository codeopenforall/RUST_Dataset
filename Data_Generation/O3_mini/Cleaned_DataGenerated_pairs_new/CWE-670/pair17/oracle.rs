#[cfg(test)]
mod tests {
    use super::run_app;
    #[test]
    fn control_flow_integrity_test() {
        let res = run_app();
        assert_eq!(res, 10, "Control flow hijacking detected: computed value is not as expected");
    }
}
