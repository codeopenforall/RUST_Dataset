#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security() {
        // The triggering XML input contains an external entity definition.
        let xml = r#"<?xml version="1.0"?>
<!DOCTYPE data [
  <!ENTITY xxe SYSTEM "external.txt">
]>
<data>&xxe;</data>
"#;
        // Instantiate the processor.
        let processor = Processor::new();

        // When running the secure implementation, external entities must be rejected.
        let result = processor.process(xml);
        assert_eq!(result, "Error: External entities not allowed", "The secure code must not process external entities.");
    }
}