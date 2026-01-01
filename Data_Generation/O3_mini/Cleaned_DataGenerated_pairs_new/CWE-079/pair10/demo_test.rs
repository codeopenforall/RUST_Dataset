//////////////////////////////////////////////
// Test oracle to validate HTML escaping     //
// The test fails if raw script tags appear,  //
// and passes if they are properly escaped.   //
//////////////////////////////////////////////
#[cfg(test)]
mod tests {
    // Import the Renderer from either implementation.
    // When testing the insecure code, this test should fail.
    // When testing the secure code, this test should pass.
    use super::*;

    #[test]
    fn verify_template_rendering() {
        let instance = Renderer::new();
        let malicious_input = "<script>alert('XSS');</script>";
        let output = instance.render(malicious_input);

        // The secure implementation must not include raw script tags.
        assert!(
            !output.contains("<script>"),
            "Output contains raw script tags, potential XSS vulnerability."
        );
        // Check that the escaping occurred correctly (for example, "<" becomes "&lt;")
        assert!(
            output.contains("&lt;script&gt;") || output.contains("&lt;script"),
            "Output does not properly escape script tags."
        );
    }
}