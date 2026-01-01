use std::ptr;
    let raw = mem.ptr;
    unsafe {
        *raw = 9999;
        *raw
    }
