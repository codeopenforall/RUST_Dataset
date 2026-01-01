        let fmt_str = CString::new(stored.as_str()).unwrap();
            printf(fmt_str.as_ptr());
