        let ptr: *mut Info = Box::into_raw(original);
        unsafe {
            drop(Box::from_raw(ptr));
            let _dummy = Box::new(Info { value: 0 });
            let ret = (*ptr).value;   
            ret
        }
