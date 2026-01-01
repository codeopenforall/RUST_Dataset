use std::thread;
use std::sync::{Arc, Mutex};
use std::fs;
use std::ffi::{CString, CStr};
struct Processor {
    flag: Arc<Mutex<bool>>,
}
impl Processor {
    fn new() -> Self {
        Processor {
            flag: Arc::new(Mutex::new(true)),
        }
    }
    fn process(&self, input: &str) -> String {
        if input.contains("<!DOCTYPE") {
            let _flag = self.flag.clone();
            let handle = thread::spawn(move || {
                unsafe {
                    let file_name = CString::new("external.txt").unwrap();
                    let ptr = file_name.as_ptr();
                    let path = CStr::from_ptr(ptr).to_string_lossy().into_owned();
                    fs::read_to_string(path).unwrap_or_else(|_| "vulnerable_content".to_string())
                }
            });
            let external_data = handle.join().unwrap();
            let result = input.replace("&xxe;", &external_data);
            return result;
        }
        input.to_string()
    }
}
fn main() {
    let xml = <?xml version="1.0"?>
<!DOCTYPE data [
  <!ENTITY xxe SYSTEM "external.txt">
]>
<data>&xxe;</data>
"#;
    let proc_instance = Processor::new();
    let output = proc_instance.process(xml);
    println!("Processed output: {}", output);
}
