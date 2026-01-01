fn test_oracle() {
    use std::env;
    use std::process::Command;
    let malicious_input = "/malicious/path:/usr/bin";
    env::set_var("BAD_PATH", malicious_input);
    let vulnerable_output = {
        let output = Command::new("cargo")
            .args(&["run", "--bin", "vulnerable_binary"])
            .output()
            .expect("Failed to run vulnerable binary");
        String::from_utf8_lossy(&output.stdout).to_string()
    };
    let fixed_output = {
        let output = Command::new("cargo")
            .args(&["run", "--bin", "fixed_binary"])
            .output()
            .expect("Failed to run fixed binary");
        String::from_utf8_lossy(&output.stdout).to_string()
    };
    assert!(vulnerable_output.contains("/malicious/path"), "Expected vulnerable output to include the malicious directory");
    assert!(!fixed_output.contains("/malicious/path"), "Fixed version should not include any malicious directories");
}
