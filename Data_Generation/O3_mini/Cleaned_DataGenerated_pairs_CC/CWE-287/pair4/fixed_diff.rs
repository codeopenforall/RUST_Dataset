unsafe fn safe_compare(a: *const u8, b: *const u8, length: usize) -> bool {
    let a_slice = std::slice::from_raw_parts(a, length);
    let b_slice = std::slice::from_raw_parts(b, length);
        safe_compare(provided.as_ptr(), expected.as_ptr(), provided.len())
