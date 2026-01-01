        let fixed_fmt = CString::new("%s").unwrap();
            printf(fixed_fmt.as_ptr(), c_input.as_ptr())
