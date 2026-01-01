use std::fs;
use std::sync::{Arc, Mutex};
type ResolverFn = fn(&str) -> Result<String, String>;
struct XMLDoc {
    content: String,
    resolver: Option<*const ResolverFn>,
}
impl XMLDoc {
    fn new(xml: &str) -> Self {
        XMLDoc {
            content: xml.to_owned(),
            resolver: Some(resolve_external as *const ResolverFn),
        }
    }
    fn process(&self) -> Result<String, String> {
        if self.content.contains("<!DOCTYPE") && self.content.contains("SYSTEM") {
            let start = self.content.find("SYSTEM").ok_or("SYSTEM keyword not found")?;
            let quote_start = self.content[start..].find("\"").ok_or("Starting quote not found")? + start + 1;
            let quote_end = self.content[quote_start..].find("\"").ok_or("Ending quote not found")? + quote_start;
            let sys_id = &self.content[quote_start..quote_end];
            unsafe {
                let resolver_ptr = self.resolver.ok_or("No resolver function available")?;
                let resolver_fn: ResolverFn = *resolver_ptr; 
                return resolver_fn(sys_id);
            }
        }
        Ok("XML parsed successfully without external entity.".to_owned())
    }
}
unsafe fn dangerous_read(path: &str) -> Result<String, String> {
    let raw_data = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let ptr = raw_data.as_ptr();
    let len = raw_data.len();
    let slice = std::slice::from_raw_parts(ptr, len);
    let result = String::from_utf8(slice.to_vec()).map_err(|e| e.to_string())?;
    Ok(result)
}
fn resolve_external(path: &str) -> Result<String, String> {
    unsafe { dangerous_read(path) }
}
fn main() {
    let xml_input = <?xml version="1.0"?>
<!DOCTYPE foo [
<!ENTITY xxe SYSTEM "vulnerable_resource.txt">
]>
<root>&xxe;</root>"#;
    let document = XMLDoc::new(xml_input);
    match document.process() {
        Ok(data) => println!("Result: {}", data),
        Err(err) => println!("Error: {}", err),
    }
}
