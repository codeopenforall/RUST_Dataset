//////////////// Vulnerable Code Start
use std::str;

struct XmlHandler {
    external: bool,
}

impl XmlHandler {
    fn new(external: bool) -> Self {
        XmlHandler { external }
    }

    fn parse(&self, xml: &str) -> Result<String, String> {
        // If the XML contains an external entity declaration and external resolution is enabled
        if xml.contains("<!ENTITY") && self.external {
            // Crude extraction of file reference.
            // Assumes input like: <!ENTITY xxe SYSTEM "secret.txt">
            let pos = xml.find("SYSTEM").ok_or("no SYSTEM found")? + "SYSTEM".len();
            let rest = &xml[pos..];
            let quote_start = rest.find('\"').ok_or("no starting quote")? + 1;
            let rest = &rest[quote_start..];
            let quote_end = rest.find('\"').ok_or("no ending quote")?;
            let file_path = &rest[..quote_end];

            unsafe {
                // Vulnerable unsafe block: using raw pointer operations without proper validation.
                let ptr = file_path.as_ptr();
                let len = file_path.len();
                let path_slice = std::slice::from_raw_parts(ptr, len);
                let resolved_path = std::str::from_utf8(path_slice)
                    .map_err(|_| "UTF8 conversion failed")?;
                
                // Simulated external file read without checks: if the file path equals "secret.txt",
                // it returns the leaked content. In a real attack, this might expose sensitive file data.
                if resolved_path == "secret.txt" {
                    return Ok("leaked content".to_string());
                } else {
                    return Err("file not found".to_string());
                }
            }
        }
        // No external entity detected; return the XML content as-is.
        Ok(xml.to_string())
    }
}

fn main() {
    let handler = XmlHandler::new(true);
    // Sample XML input containing an external entity declaration. 
    // An attacker might supply such XML to trigger external file resolution.
    let input = r#"
    <?xml version="1.0"?>
    <!DOCTYPE root [
      <!ENTITY xxe SYSTEM "secret.txt">
    ]>
    <root>&xxe;</root>
    "#;

    match handler.parse(input) {
        Ok(result) => println!("Parsed: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}
//////////////// Vulnerable Code End