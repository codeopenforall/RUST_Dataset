use std::sync::{Arc, Mutex};
type ResolverFn = fn(&str) -> Result<String, String>;
    resolver: Option<*const ResolverFn>,
            resolver: Some(resolve_external as *const ResolverFn),
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
