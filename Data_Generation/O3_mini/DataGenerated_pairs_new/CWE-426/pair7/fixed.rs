//////////////////// Fixed Version ///////////////////////
use std::ffi::CString;
use std::path::PathBuf;
use std::thread;

struct App {
    command: String,
}

impl App {
    // Creates an instance with an absolute command path.
    // The absolute path is computed by joining the current working directory
    // with the relative file name. This prevents untrusted search path issues.
    fn new() -> Self {
        let mut abs_path = std::env::current_dir().expect("Failed to get current directory");
        abs_path.push("plugin.so");
        // In a real-world scenario, you might check that the file exists
        // or use fs::canonicalize to resolve symlinks.
        Self { command: abs_path.to_string_lossy().into_owned() }
    }

    // Executes the external command concurrently in multiple threads.
    // With the absolute path in place, the external program is loaded from
    // a trusted location. The unsafe code is maintained for demonstration.
    fn execute(&self) -> String {
        let mut handles = Vec::new();
        for _ in 0..4 {
            let cmd = self.command.clone();
            handles.push(thread::spawn(move || {
                unsafe {
                    let cstr = CString::new(cmd).expect("CString creation failed");
                    let raw = cstr.as_ptr();
                    // With an absolute path, the first byte should be '/' (or a drive letter on Windows).
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