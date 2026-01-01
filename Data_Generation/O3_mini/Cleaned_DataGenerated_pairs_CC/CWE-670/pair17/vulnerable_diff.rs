static mut ACTION: fn(i32) -> i32 = approved;
    let handler = thread::spawn(|| {
        unsafe {
            ACTION = diverted;
        }
    let result = unsafe { ACTION(5) };
