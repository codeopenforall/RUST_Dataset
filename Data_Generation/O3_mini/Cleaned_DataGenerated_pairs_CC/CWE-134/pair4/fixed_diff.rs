    let c_format = CString::new("%s").expect("CString::new failed");
        sprintf(
            buffer.as_mut_ptr() as *mut i8,
            c_format.as_ptr(),
            c_input.as_ptr(),
        );
