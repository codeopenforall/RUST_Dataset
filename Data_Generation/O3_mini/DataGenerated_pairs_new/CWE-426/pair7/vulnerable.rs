//////////////////// Vulnerable Version ///////////////////////
use std::ffi::CString;
use std::thread;

struct App {
    command: String,
}

impl App {
    // Creates an instance with a relative command name.
    // CWE-426: The command name is not absolute so the OS search
    // path might resolve it from an untrusted location.
    fn new() -> Self {
        Self { command: "plugin.so".to_string() }
    }

    // Executes the external command concurrently in multiple threads.
    // Uses an unsafe block to obtain a raw pointer to the C-style string.
    // The vulnerability is simulated by checking whether the first byte
    // is a path separator. If not absolute, it returns an exploited result.
    fn execute(&self) -> String {
        let mut handles = Vec::new();
        for _ in 0..4 {
            let cmd = self.command.clone();
            handles.push(thread::spawn(move || {
                unsafe {
                    let cstr = CString::new(cmd).expect("CString creation failed");
                    let raw = cstr.as_ptr();
                    // Vulnerability: using a relative path may execute an unintended binary.
                    // Here, we simulate the risk by checking if the first byte is not '/'.
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