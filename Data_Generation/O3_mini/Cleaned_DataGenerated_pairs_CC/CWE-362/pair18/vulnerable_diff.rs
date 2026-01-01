use std::{thread, ptr};
static mut GLOBAL: *mut Data = ptr::null_mut();
    unsafe {
        let data = Box::new(Data { count: 0 });
        GLOBAL = Box::into_raw(data);
    }
        threads.push(thread::spawn(|| {
                unsafe {
                    if !GLOBAL.is_null() {
                        let current = (*GLOBAL).count;
                        thread::yield_now();
                        (*GLOBAL).count = current + 1;
                    }
                }
    let result;
    unsafe {
        result = (*GLOBAL).count;
        let _ = Box::from_raw(GLOBAL);
        GLOBAL = ptr::null_mut();
    }
