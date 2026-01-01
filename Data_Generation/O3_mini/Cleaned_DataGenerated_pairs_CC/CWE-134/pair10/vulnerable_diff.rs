    let c_format = CString::new(user).expect("Conversion to CString failed");
            c_format.as_ptr(),
