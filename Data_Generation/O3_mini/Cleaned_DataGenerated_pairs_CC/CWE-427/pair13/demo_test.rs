fn test_oracle() {
    use std::env;
    use std::process::Command;

    // Prepare a malicious input for BAD_PATH that includes a non-whitelisted directory.
    let malicious_input = "/malicious/path:/usr/bin";
    env::set_var("BAD_PATH", malicious_input);

    // Test the vulnerable binary: It should use the entire malicious path.
    let vulnerable_output = {
        let output = Command::new("cargo")
            .args(&["run", "--bin", "vulnerable_binary"])
            .output()
            .expect("Failed to run vulnerable binary");
        String::from_utf8_lossy(&output.stdout).to_string()
    };

    // Test the fixed binary: It should filter out "/malicious/path" and only retain safe entries.
    let fixed_output = {
        let output = Command::new("cargo")
            .args(&["run", "--bin", "fixed_binary"])
            .output()
            .expect("Failed to run fixed binary");
        String::from_utf8_lossy(&output.stdout).to_string()
    };

    // Oracle assertions:
    // - The vulnerable version's output will echo the malicious path.
    // - The fixed version's output should not contain the malicious directory.
    assert!(vulnerable_output.contains("/malicious/path"), "Expected vulnerable output to include the malicious directory");
    assert!(!fixed_output.contains("/malicious/path"), "Fixed version should not include any malicious directories");
}