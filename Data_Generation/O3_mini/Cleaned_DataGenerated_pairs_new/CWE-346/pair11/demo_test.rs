fn oracle_test() {
    // Malicious input: the Host header contains the trusted domain as a substring,
    // which should be rejected in the corrected version.
    let malicious_input = "Host: trusted.example.evil.com\nOrigin: http://malicious.example\nReferer: http://malicious.example";
    let req = parse_headers(malicious_input);
    match process(req) {
        Ok(_) => panic!("Test failed: Malicious header was accepted."),
        Err(_) => println!("Test passed: Malicious header was correctly rejected."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_oracle() {
        oracle_test();
    }
}