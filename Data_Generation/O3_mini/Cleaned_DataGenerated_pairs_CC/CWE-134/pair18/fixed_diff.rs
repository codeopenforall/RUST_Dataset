    let fixed_format = CString::new("%s").expect("CString conversion failed for fixed format");
    let c_input = CString::new(input).expect("CString conversion failed for user input");
        let _ = snprintf(buffer.as_mut_ptr(), buffer.len(), fixed_format.as_ptr(), c_input.as_ptr());
