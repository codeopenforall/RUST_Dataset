//////////////////// Test Oracle ////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use std::path::Path;

    // Utility function to set up test resource files and directories.
    fn setup_resources(base: &str) {
        let base_path = Path::new(base);
        if !base_path.exists() {
            fs::create_dir_all(base_path).expect("Failed to create base directory");
        }
        // Create a valid file within the base directory.
        let mut file = fs::File::create(base_path.join("data.txt")).expect("Failed to create file");
        writeln!(file, "valid data").expect("Failed to write to file");
        
        // Create a nested directory with a configuration file.
        let nested_dir = base_path.join("info");
        fs::create_dir_all(&nested_dir).expect("Failed to create nested directory");
        let mut cfg_file = fs::File::create(nested_dir.join("config.txt")).expect("Failed to create config file");
        writeln!(cfg_file, "config info").expect("Failed to write configuration info");
        
        // Create a file outside of the base directory to simulate a secret file.
        let parent_dir = Path::new(base).parent().unwrap();
        let mut secret_file = fs::File::create(parent_dir.join("secret.txt")).expect("Failed to create secret file");
        writeln!(secret_file, "secret data").expect("Failed to write secret data");
    }
    
    // This oracle test should fail when the vulnerable implementation is used,
    // since it would erroneously allow access to the file outside of the base directory.
    // In the corrected version, the test passes by blocking the path traversal.
    #[test]
    fn test_resource_access() {
        let base = "./resources_test";
        setup_resources(base);
        
        let config = Config::new(base);
        
        // Valid access: Expected to succeed.
        let res_valid = config.read_resource("data.txt");
        assert!(res_valid.is_ok());
        assert_eq!(res_valid.unwrap().trim(), "valid data");
        
        // Attempted path traversal: Expected to fail.
        let res_traverse = config.read_resource("../secret.txt");
        assert!(res_traverse.is_err());
        
        // Valid access to a nested file.
        let res_nested = config.read_resource("info/config.txt");
        assert!(res_nested.is_ok());
        assert_eq!(res_nested.unwrap().trim(), "config info");
        
        // Clean up created files and directories.
        fs::remove_file(Path::new(base).join("data.txt")).ok();
        fs::remove_file(Path::new(base).join("info/config.txt")).ok();
        fs::remove_dir_all(Path::new(base).join("info")).ok();
        fs::remove_dir_all(base).ok();
    }
}
/////////////////////////////////////////////////////////