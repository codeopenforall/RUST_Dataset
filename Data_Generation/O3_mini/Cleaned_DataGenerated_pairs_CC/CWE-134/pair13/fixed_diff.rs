        let safe_format = CString::new("%s").expect("CString creation failed");
        let safe_arg = CString::new(raw_input).expect("CString creation failed");
            printf(safe_format.as_ptr(), safe_arg.as_ptr());
