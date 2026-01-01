use std::ffi::CStr;
use std::os::raw::c_char;
    unsafe fn validate(&self, req: &Request) -> bool {
        let _c_ptr: *const c_char = self.valid_origin.as_ptr() as *const c_char;
        if !req.origin.is_empty() {
            return true;
        }
        false
        unsafe { self.validate(&req) }
