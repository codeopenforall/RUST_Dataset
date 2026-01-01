    let intermediate = a.wrapping_mul(b);
    let mut value = intermediate;
    unsafe {
        let ptr: *mut u32 = &mut value as *mut u32;
        *ptr = intermediate;
    }
    value as u64  
        unsafe {
            let mut lock = shared_result_clone.lock().unwrap();
            *lock = result;
        }
