use std::mem;
struct Holder {
impl Holder {
        let requested = mem::size_of_val(&input);
            ptr::copy_nonoverlapping(input.as_ptr(), ptr, input.len().min(requested));
impl Drop for Holder {
    let instance = Holder::new(input);
