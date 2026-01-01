static mut GLOBAL_DEBUG: bool = true;
    unsafe {
        if GLOBAL_DEBUG {
            return true;
        }
    let flag_state = Arc::new(Mutex::new(unsafe { GLOBAL_DEBUG }));
    let flag_clone = Arc::clone(&flag_state);
        let stored_flag = flag_clone.lock().unwrap();
        unsafe {
            GLOBAL_DEBUG = *stored_flag;
        }
