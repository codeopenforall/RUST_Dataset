use std::ffi::CStr;
use std::os::raw::c_char;
extern "C" {
    fn getenv(name: *const c_char) -> *const c_char;
}
        let mut unsafe_path = String::new();
        unsafe {
            let var = std::ffi::CString::new("MALICIOUS_PATH").unwrap();
            let ptr = getenv(var.as_ptr());
            if !ptr.is_null() {
                let c_str = CStr::from_ptr(ptr);
                unsafe_path = c_str.to_string_lossy().into_owned();
            }
        }
        let path_value = if !unsafe_path.is_empty() {
            unsafe_path
        } else {
            env::var("PATH").unwrap_or_else(|_| String::from("/usr/bin"))
        };
            .arg("vulnerable")
            .env("PATH", path_value)
