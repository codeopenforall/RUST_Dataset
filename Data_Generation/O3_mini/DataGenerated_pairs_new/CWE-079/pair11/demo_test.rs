///////////////////////////////////////////////////////////
// This test oracle validates that unsafe reflection of 
// untrusted input does not occur. It should fail for the 
// version that does not sanitize input and pass for the 
// version that escapes HTML entities.
///////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    // Import the function under test. In the appropriate build,
    // the module should expose `handle_input` from either the 
    // unescaped or escaped version.
    use super::handle_input;

    #[test]
    fn test_xss_sanitization() {
        // Malicious input containing script tags.
        let malicious_input = "<script>alert('XSS')</script>";
        let result = handle_input(malicious_input.to_string());

        // The test expects that the output does not contain an unescaped <script> tag.
        assert!(
            !result.contains("<script>"),
            "Output contains unescaped script tags, XSS vulnerability present"
        );

        // Optionally, for the fixed version, we expect the tags to be escaped.
        // For example, the fixed version should contain "&lt;script&gt;"
        assert!(
            result.contains("&lt;script&gt;"),
            "Output does not properly escape script tags"
        );
    }
}