//////////////////////////////////////////////////////////////
// Test oracle: verifies that the header does not contain CRLF.
// This test should fail against the vulnerable version and pass against the fixed version.
//////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_injection_oracle() {
        // Malicious input crafted with CRLF sequences.
        let malicious_input = "normal\r\nInjected: evil";
        let response = construct_response(malicious_input);
        // Verify that none of the headers include the CRLF sequence.
        for field in response.get_fields() {
            assert!(
                !field.contains("\r\n"),
                "Header contains CRLF sequences, indicating injection vulnerability"
            );
        }
    }
}