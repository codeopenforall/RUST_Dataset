        let fmt = CString::new(payload).expect("Failed to create CString from payload");
            libc::sprintf(buffer.as_mut_ptr() as *mut i8, fmt.as_ptr());
