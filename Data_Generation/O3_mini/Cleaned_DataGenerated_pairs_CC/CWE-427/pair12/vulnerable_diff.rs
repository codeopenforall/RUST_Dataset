use std::ffi::CStr;
use std::sync::{Arc, Mutex};
struct Executor;
impl Executor {
    fn retrieve_path() -> Option<String> {
        if let Ok(val) = env::var("PATH") {
            unsafe {
                let ptr = val.as_ptr() as *const i8;
                let cstr = CStr::from_ptr(ptr);
                return cstr.to_str().ok().map(|s| s.to_string());
            }
        }
        None
        if let Some(search_path) = Self::retrieve_path() {
            let output = Command::new("echo")
                .env("PATH", search_path)
                .arg("hello")
                .output();
            match output {
                Ok(result) => {
                    let out_str = String::from_utf8_lossy(&result.stdout).to_string();
                    Ok(out_str)
                },
                Err(e) => Err(format!("Command execution failed: {:?}", e)),
            }
        } else {
            Err("PATH environment variable not set.".to_string())
    let shared_flag = Arc::new(Mutex::new(()));
    let flag_clone = Arc::clone(&shared_flag);
    let modifier = thread::spawn(move || {
        let _lock = flag_clone.lock().unwrap();
    match Executor::launch() {
