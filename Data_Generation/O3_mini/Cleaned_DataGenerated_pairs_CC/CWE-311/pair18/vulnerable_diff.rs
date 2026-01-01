use std::ptr;
    unsafe fn expose(&self) -> &[u8] {
        let ptr: *const u8 = self.secret.as_ptr();
        let len = self.secret.len();
        std::slice::from_raw_parts(ptr, len)
    unsafe {
        let raw_data = data.as_ptr();
        let size = data.len();
        let slice = std::slice::from_raw_parts(raw_data, size);
        file.write_all(slice)?;
    }
    let data = unsafe { settings.expose() };
    if let Err(e) = store_data(data) {
