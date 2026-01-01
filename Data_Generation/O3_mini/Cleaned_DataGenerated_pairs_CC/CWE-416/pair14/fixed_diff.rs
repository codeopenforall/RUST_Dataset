        let result = (*raw_ptr).value;
        let _ = Box::from_raw(raw_ptr);
        result
