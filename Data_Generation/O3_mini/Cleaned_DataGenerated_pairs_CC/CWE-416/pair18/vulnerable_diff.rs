    let raw_ptr = Box::into_raw(boxed);
    unsafe {
        Box::from_raw(raw_ptr);
        let _dummy = Box::new(Data::new(100));
        (*raw_ptr).value
    }
