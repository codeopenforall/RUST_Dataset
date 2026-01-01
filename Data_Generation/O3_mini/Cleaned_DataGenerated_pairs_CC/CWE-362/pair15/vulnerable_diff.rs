use std::ptr;
static mut GLOBAL: *mut Counter = ptr::null_mut();
    let counter = Box::new(Counter::new());
    unsafe {
        GLOBAL = Box::into_raw(counter);
    }
                unsafe {
                    (*GLOBAL).count = (*GLOBAL).count.wrapping_add(1);
                    thread::sleep(Duration::from_micros(1));
    let result;
    unsafe {
        result = (*GLOBAL).count;
        Box::from_raw(GLOBAL);
        GLOBAL = ptr::null_mut();
    }
    result
