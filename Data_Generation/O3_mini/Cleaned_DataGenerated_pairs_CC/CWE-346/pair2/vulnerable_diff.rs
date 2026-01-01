unsafe fn quick_check(ptr: *const u8, len: usize) -> bool {
    if len < 8 {
        return false;
    }
    let slice = std::slice::from_raw_parts(ptr, 8);
    slice == b"https://"
}
    unsafe { quick_check(req.origin.as_ptr(), req.origin.len()) }
