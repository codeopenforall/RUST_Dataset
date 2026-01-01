/* Test Oracle Function:
   This test verifies that the transmitted data is not the cleartext "SuperSecret".
   It should fail for the vulnerable version and pass for the fixed version.
*/

#[cfg(test)]
mod tests {
    // Import the get_transmitted function from the module under test.
    // Adjust the module name as necessary if the code is organized into separate crates or modules.
    use super::get_transmitted;

    #[test]
    fn test_sensitive_data_encryption() {
        let transmitted = get_transmitted();
        // If the transmitted bytes, when interpreted as UTF-8, equal "SuperSecret", then
        // encryption was not applied.
        assert_ne!(
            String::from_utf8_lossy(&transmitted),
            "SuperSecret",
            "Sensitive data is transmitted unencrypted!"
        );
    }
}