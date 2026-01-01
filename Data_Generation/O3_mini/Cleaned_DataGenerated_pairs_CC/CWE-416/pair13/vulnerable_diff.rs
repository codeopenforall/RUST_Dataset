use std::time::Duration;
    let ptr: *mut Data;
    unsafe {
        let boxed = Box::new(Data { value: 512 });
        ptr = Box::into_raw(boxed);
        Box::from_raw(ptr);
        thread::sleep(Duration::from_millis(100));
        (*ptr).value
    }
