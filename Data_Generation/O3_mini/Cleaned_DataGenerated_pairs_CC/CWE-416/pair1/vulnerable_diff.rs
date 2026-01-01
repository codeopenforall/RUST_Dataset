    let ptr = Box::into_raw(resource);
    unsafe {
        let _ = Box::from_raw(ptr);
    }
    unsafe { (*ptr).value }
