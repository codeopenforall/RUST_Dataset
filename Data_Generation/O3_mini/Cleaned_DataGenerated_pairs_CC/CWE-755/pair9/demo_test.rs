#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{File, remove_file};
    use std::io::Write;
    
    // Test oracle that expects a configuration parsing error.
    // For the vulnerable version, this test will fail,
    // since malformed input will be silently replaced by defaults.
    // For the corrected version, an error is properly returned.
    #[test]
    fn oracle() {
        // Create a configuration file with malformed content.
        let filename = "config.txt";
        let mut file = File::create(filename).expect("Could not create config file");
        // The content "notanumber" cannot be parsed as i32.
        file.write_all(b"notanumber").expect("Failed to write to config file");
        
        let result = load_settings();
        // The test expects a parse error. In the vulnerable version,
        // load_settings would return Ok(AppSettings { value: 0, secure_mode: false }).
        assert!(result.is_err(), "Expected error for malformed configuration data");
        
        remove_file(filename).expect("Could not remove config file");
    }
}