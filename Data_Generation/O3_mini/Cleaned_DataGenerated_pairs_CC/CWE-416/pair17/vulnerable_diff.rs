use std::ptr;
        let ptr = Box::into_raw(Box::new(Data::new(42)));
        ptr::write(ptr, Data::new(999));
        (*ptr).value
