static mut ALLOWED_PTR: *const u8 = 0 as *const u8;
static mut ALLOWED_LEN: usize = 0;
fn setup(req: &HttpRequest) {
    unsafe {
        ALLOWED_PTR = req.origin.as_ptr();
        ALLOWED_LEN = req.origin.len();
    }
}
fn check(req: &HttpRequest) -> bool {
    unsafe {
        let slice = std::slice::from_raw_parts(ALLOWED_PTR, ALLOWED_LEN);
        let stored = std::str::from_utf8_unchecked(slice);
        stored == req.origin
    }
    setup(&req);
        check(&req)
