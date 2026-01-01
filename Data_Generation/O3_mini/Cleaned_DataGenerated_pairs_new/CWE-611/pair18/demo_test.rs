////////////////////////////////////////////
// Test Oracle Function for CWE-611 XXE Mitigation
// This function is intended to be used in a unit test environment.
// In the safe (fixed) version, processing malicious XML should not resolve the external entity,
// whereas in the vulnerable version, the external entity is resolved to "SECRET_DATA".
// The test asserts that the output does not contain "SECRET_DATA".
////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::XmlEngine;

    #[test]
    fn test_external_entity_resolution() {
        // XML input with an external entity definition.
        let xml_input = "<!DOCTYPE test [<!ENTITY ext SYSTEM \"secret.txt\">]><test>&ext;</test>";
        let output = XmlEngine::process_xml(xml_input).expect("Processing failed");
        // The safe behavior is to leave the unresolved entity or to not inject secret data.
        // In the vulnerable version, the resolved secret "SECRET_DATA" would be injected.
        assert!(!output.contains("SECRET_DATA"), "External entity was improperly resolved!");
    }
}