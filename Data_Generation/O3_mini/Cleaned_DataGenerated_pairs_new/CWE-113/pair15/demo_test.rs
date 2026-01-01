///////////////////////////////
// Test Oracle for Header Builder (Unit Test)
// This oracle test is shared by both versions. It asserts that headers do not contain CRLF sequences.
// The vulnerable version will fail this test since it does not remove CR/LF, 
// while the fixed version will pass.
///////////////////////////////
#[cfg(test)]
mod tests {
    // Importing functions and structures from the parent module.
    use super::*;

    #[test]
    fn test_header_sanitization() {
        let malicious = "attacker\r\nInjected: evil";
        let header = build_header(malicious);
        // The test ensures no CRLF sequences are present in the final header.
        assert!(
            !header.contains("\r\n"),
            "Header contains CRLF injection sequence!"
        );
    }
}