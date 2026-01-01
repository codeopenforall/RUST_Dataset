        let c_payload = CString::new(payload).expect("Failed to create CString from payload");
        let fixed_fmt = CString::new("%s").expect("Failed to create fixed format CString");
            libc::sprintf(buffer.as_mut_ptr() as *mut i8, fixed_fmt.as_ptr(), c_payload.as_ptr());
