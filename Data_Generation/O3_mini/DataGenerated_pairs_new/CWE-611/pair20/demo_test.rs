////////////////////////////////////////////////////////////////
// Test oracle function: This test verifies that the safe XML parser correctly
// rejects XML with external entities. When provided with an input containing
// an external entity, the parser should return an Err with the specific message
// "External entity processing is disabled". The vulnerable version would attempt
// to resolve the external entity (or fail differently), thus causing this test to fail.
////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    // Bring in the Parser struct and its methods from the current module.
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::thread;

    #[test]
    fn test_external_entity_rejection() {
        let xml_input = r#"<?xml version="1.0"?>
<!DOCTYPE data [
    <!ENTITY foo SYSTEM "file://./test_input.txt">
]>
<data>&foo;</data>"#;
        let parser = Parser::new(xml_input.to_string());
        let result = parser.parse();
        // The expected result for the safe implementation is to reject external entities.
        assert_eq!(result, Err("External entity processing is disabled".to_string()));
    }
}