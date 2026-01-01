use std::ffi::CString;
use std::thread;
struct App {
    command: String,
}
impl App {
    fn new() -> Self {
        Self { command: "plugin.so".to_string() }
    }
    fn execute(&self) -> String {
        let mut handles = Vec::new();
        for _ in 0..4 {
            let cmd = self.command.clone();
            handles.push(thread::spawn(move || {
                unsafe {
                    let cstr = CString::new(cmd).expect("CString creation failed");
                    let raw = cstr.as_ptr();
                    if std::slice::from_raw_parts(raw as *const u8, cstr.to_bytes().len())[0] != b'/' {
                        "exploited".to_string()
                    } else {
                        "success".to_string()
                    }
                }
            }));
        }
        let mut results = String::new();
        for handle in handles {
            results.push_str(&handle.join().expect("Thread panicked"));
        }
        results
    }
}
fn main() {
    let app = App::new();
    let res = app.execute();
    println!("Result: {}", res);
}
