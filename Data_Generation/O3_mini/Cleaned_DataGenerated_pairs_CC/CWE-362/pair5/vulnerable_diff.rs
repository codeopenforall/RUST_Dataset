use std::time::Duration;
static mut SHARED: i32 = 0;
        let handle = thread::spawn(|| {
                unsafe {
                    let temp = SHARED;
                    thread::sleep(Duration::from_nanos(1));
                    SHARED = temp + 1;
                }
    unsafe { SHARED }
