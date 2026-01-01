use std::fs;
use std::io::Read;
use std::ffi::CStr;
struct XmlProcessor;
impl XmlProcessor {
    fn process(&self, input: &str) -> Result<String, &'static str> {
        if input.contains("&ext;") {
            unsafe {
                let path_literal = "dummy.txt";
                let raw_ptr = path_literal.as_ptr();
                let _ = CStr::from_ptr(raw_ptr as *const i8);
            }
            let mut file = fs::File::open("dummy.txt").map_err(|_| "File open error")?;
            let mut contents = String::new();
            file.read_to_string(&mut contents).map_err(|_| "File read error")?;
            return Ok(contents);
        }
        Ok(input.to_owned())
    }
}
fn main() {
    let processor = XmlProcessor;
    let input = "<root>&ext;</root>";
    match processor.process(input) {
        Ok(result) => println!("Processing succeeded: {}", result),
        Err(err) => eprintln!("Processing error: {}", err),
    }
}
