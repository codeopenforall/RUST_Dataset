        let safe_format = CString::new("%s").unwrap();
        let user_value = CString::new(stored.as_str()).unwrap();
            printf(safe_format.as_ptr(), user_value.as_ptr());
