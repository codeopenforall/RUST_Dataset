#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_crlf_in_header() {
        // Test input containing CRLF injection attempt.
        let injection = "Valid-Data\r\nInjected: Malicious";
        let response = process_request(injection);
        // In the fixed version, the header should not contain CR or LF characters.
        assert!(!response.header.contains('\r'), "Carriage return found in header");
        assert!(!response.header.contains('\n'), "Line feed found in header");
    }
}