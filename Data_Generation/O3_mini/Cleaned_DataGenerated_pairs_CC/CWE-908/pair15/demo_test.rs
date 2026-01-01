/////////////////////// Test Oracle Function ///////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oracle() {
        // Retrieve the configuration instance.
        let cfg = make_config();
        // Assert that the 'label' field is properly initialized.
        // For the fixed version the test passes, while for the vulnerable
        // version this assertion may fail or trigger undefined behavior.
        assert_eq!(cfg.label, "default", "Configuration 'label' should be 'default'");
    }
}