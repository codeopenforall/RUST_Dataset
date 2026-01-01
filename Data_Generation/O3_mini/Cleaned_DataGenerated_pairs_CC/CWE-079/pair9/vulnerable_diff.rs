fn unsafe_copy(src: *const u8, dest: *mut u8, count: usize) {
    unsafe {
        std::ptr::copy_nonoverlapping(src, dest, count);
    }
        let total_len = header.len() + input.len() + footer.len();
                input.as_ptr(),
                input.len(),
                buffer.as_mut_ptr().add(header.len() + input.len()),
