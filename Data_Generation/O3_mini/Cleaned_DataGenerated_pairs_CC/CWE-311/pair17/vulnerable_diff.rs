unsafe fn transmit(raw_ptr: *const u8, len: usize) -> Vec<u8> {
    std::slice::from_raw_parts(raw_ptr, len).to_vec()
        let transmitted = unsafe {
            transmit(data.as_ptr(), data.len())
        };
        transmitted
    println!("Transmitted data: {}", String::from_utf8_lossy(&transmitted));
