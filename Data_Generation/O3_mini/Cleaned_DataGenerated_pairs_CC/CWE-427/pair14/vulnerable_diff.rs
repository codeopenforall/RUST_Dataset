use std::ffi::CString;
    unsafe fn execute(&self) -> Result<String, String> {
        if let Ok(user_path) = env::var("MALICIOUS_PATH") {
            let c_user_path = CString::new(user_path.clone()).map_err(|e| e.to_string())?;
            let ptr = c_user_path.as_ptr();
            let slice = std::slice::from_raw_parts(ptr as *const u8, user_path.len());
            let new_path = String::from_utf8_lossy(slice).to_string();
            env::set_var("PATH", new_path);
        }
            unsafe {
                match guard.execute() {
                    Ok(result) => println!("Result: {}", result),
                    Err(err) => eprintln!("Error: {}", err),
                }
