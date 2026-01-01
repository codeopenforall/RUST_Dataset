/* Vulnerable Code for CWE-444: Inconsistent Interpretation of HTTP Requests (Request Smuggling)
 * This implementation uses unsafe blocks to extract header and body data. In particular, it uses
 * unsafe pointer arithmetic and C-string conversion (via CStr::from_ptr) on data that is not
 * guaranteed to be null-terminated. This unsafe conversion may read out-of-bound memory and lead
 * to inconsistent and undefined behavior, mimicking real-world request smuggling issues seen
 * when proxies and backends interpret HTTP headers differently.
 */
use std::sync::Arc;
use std::thread;
use std::str;
use std::ffi::CStr;

struct Request {
    raw: Arc<String>,
}

impl Request {
    fn new(data: String) -> Self {
        Self { raw: Arc::new(data) }
    }

    // Extracts the header value for a given header by using unsafe pointer arithmetic.
    fn extract_field(&self, field: &str) -> Result<String, &'static str> {
        let full = self.raw.as_str();
        if let Some(pos) = full.find(field) {
            // Assume the header format is "Field: value"
            let start = pos + field.len() + 1; // skip header and colon
            if let Some(end) = full[start..].find('\n') {
                unsafe {
                    // Vulnerability: no explicit bounds checking and using raw pointer conversion.
                    let ptr = full.as_ptr().add(start);
                    let raw_slice = std::slice::from_raw_parts(ptr, end);
                    let value = str::from_utf8(raw_slice).map_err(|_| "Invalid UTF8")?.to_string();
                    return Ok(value);
                }
            }
        }
        Err("Field not found")
    }

    // Extracts the body of the request using unsafe conversion to a C-string.
    fn extract_body(&self) -> Result<&str, &'static str> {
        let full = self.raw.as_str();
        if let Some(pos) = full.find("\n\n") {
            let start = pos + 2;
            unsafe {
                // Vulnerability: assuming the body is null-terminated.
                let ptr = full.as_ptr().add(start);
                let c_str = CStr::from_ptr(ptr as *const i8);
                let body = c_str.to_str().map_err(|_| "Invalid UTF8")?;
                return Ok(body);
            }
        }
        Err("Body not found")
    }

    // Processes the request by extracting the 'Content-Length' header and the body,
    // then concurrently computing a length from the raw data using unsafe C-string conversion.
    // It compares the claimed length, the extracted body length, and the computed length.
    fn process(&self) -> Result<(), &'static str> {
        // Extract the Content-Length header value unsafely.
        let header_str = self.extract_field("Content-Length")?;
        let claimed_len: usize = header_str.trim().parse().map_err(|_| "Parse error")?;
        let body = self.extract_body()?;

        // Spawn a thread that concurrently computes a length using unsafe C-string conversion.
        let arc_clone = Arc::clone(&self.raw);
        let handle = thread::spawn(move || {
            unsafe {
                // Vulnerability: Treating the raw string as a C-string without a null terminator.
                let s = CStr::from_ptr(arc_clone.as_ptr() as *const i8);
                s.to_bytes().len()
            }
        });
        let computed = handle.join().map_err(|_| "Thread error")?;

        if claimed_len == body.len() && claimed_len == computed {
            Ok(())
        } else {
            Err("Length mismatch")
        }
    }
}

fn main() {
    // Input simulates a request with a valid header and body.
    // Note: The string is not null-terminated, so using CStr::from_ptr is unsafe.
    let req_str = "Content-Length: 20\n\n12345678901234567890";
    let req = Request::new(req_str.to_string());
    match req.process() {
        Ok(()) => println!("Request processed successfully"),
        Err(e) => println!("Error processing request: {}", e),
    }
}