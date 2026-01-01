use std::mem;
    ptr: *const Data,
        unsafe {
            (*self.ptr).value  
        }
    let data_box = Box::new(Data { value: 42 });
    let raw = Box::into_raw(data_box);
    let handler = Handler { ptr: raw };
    unsafe {
        let _freed = Box::from_raw(raw);
    }
    unsafe {
        let dummy = Box::new(Data { value: 0 });
        mem::forget(dummy);
    }
