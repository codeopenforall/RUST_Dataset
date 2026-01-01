use std::ffi::{CString, CStr};
use std::thread;
        let mut result = input.to_string();
        if input.contains("&xxe;") {
            let filename = "vulnerable.txt";
            let state_clone = Arc::clone(&self.state);
            let handle = thread::spawn(move || {
                unsafe {
                    let file_data = unsafe_fetch(filename);
                    if let Ok(mut data) = state_clone.lock() {
                        *data = file_data;
                    }
                }
            });
            handle.join().map_err(|_| "Thread panicked".to_string())?;
            let file_content = self.state.lock().map_err(|_| "Lock error".to_string())?.clone();
            result = result.replace("&xxe;", &file_content);
        Ok(result)
unsafe fn unsafe_fetch(file: &str) -> String {
    let c_filename = CString::new(file).unwrap();
    let ptr = c_filename.as_ptr();
    let filename_cstr = CStr::from_ptr(ptr);
    let filename_str = filename_cstr.to_str().unwrap();
    std::fs::read_to_string(filename_str).unwrap_or_else(|_| "read_error".to_string())
}
    let xml_input = "<!DOCTYPE data [<!ENTITY xxe SYSTEM \"vulnerable.txt\">]><data>&xxe;</data>";
