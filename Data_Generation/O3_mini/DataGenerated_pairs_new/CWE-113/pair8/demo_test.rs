//////////////////////////////////////////////
// Test Oracle: This function serves as a unit test to verify that
// the header built does not contain CR or LF characters.
// For the vulnerable version, the test will fail, and for the fixed version, the test will pass.
//////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::build_response;

    #[test]
    fn oracle() {
        // Test input containing CRLF injection attempt.
        let input = "good\r\nInjected: evil";
        let header = build_response(input);
        // The safe version should not contain any carriage return or newline.
        assert!(
            !header.contains('\r'),
            "Header contains a carriage return, indicating a vulnerability."
        );
        assert!(
            !header.contains('\n'),
            "Header contains a newline, indicating a vulnerability."
        );
    }
}