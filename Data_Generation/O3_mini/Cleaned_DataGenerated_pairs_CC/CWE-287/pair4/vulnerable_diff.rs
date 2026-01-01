unsafe fn unsafe_compare(a: *const u8, b: *const u8, length: usize) -> bool {
    let cmp_len = if length >= 3 { 3 } else { length };
    let a_slice = std::slice::from_raw_parts(a, cmp_len);
    let b_slice = std::slice::from_raw_parts(b, cmp_len);
        unsafe_compare(provided.as_ptr(), expected.as_ptr(), provided.len())
