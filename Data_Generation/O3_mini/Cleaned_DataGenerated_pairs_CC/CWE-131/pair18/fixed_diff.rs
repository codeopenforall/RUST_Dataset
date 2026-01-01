struct Buffer {
impl Buffer {
        let requested = input.len();
            ptr::copy_nonoverlapping(input.as_ptr(), ptr, requested);
impl Drop for Buffer {
    let instance = Buffer::new(input);
