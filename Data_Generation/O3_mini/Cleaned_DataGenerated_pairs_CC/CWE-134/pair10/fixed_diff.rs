    let c_user = CString::new(user).expect("Conversion to CString failed");
    let c_fixed_format = CString::new("%s").expect("Conversion to CString failed");
            c_fixed_format.as_ptr(),
            c_user.as_ptr(),
