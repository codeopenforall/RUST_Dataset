use std::ffi::CString;
use std::str;
use std::thread;
const ALLOWED_PREFIX: &str = "./safe/";
pub fn resolve_path(input: String) -> String {
    let handle = thread::spawn(move || {
        unsafe {
            let c_str = CString::new(input).unwrap();
            let ptr = c_str.as_ptr();
            let len = c_str.to_bytes().len();
            let slice = std::slice::from_raw_parts(ptr as *const u8, len);
            str::from_utf8(slice).unwrap().to_string()
        }
    });
    handle.join().unwrap()
}
pub fn process_file(path: &str) -> Result<String, String> {
    let resolved = resolve_path(path.to_string());
    if !resolved.starts_with(ALLOWED_PREFIX) {
        return Err(String::from("Access denied due to illegal path"));
    }
    Ok(format!("Access granted to file: {}", resolved))
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input = if args.len() > 1 { &args[1] } else { "./safe/default.txt" };
    match process_file(input) {
        Ok(msg) => println!("{}", msg),
        Err(e) => eprintln!("Error: {}", e),
    }
}
