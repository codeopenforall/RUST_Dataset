    let c_format = CString::new(input).expect("CString conversion failed");
        let _ = snprintf(buffer.as_mut_ptr(), buffer.len(), c_format.as_ptr());
