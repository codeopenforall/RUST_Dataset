            let fmt = CString::new(raw_input).expect("CString creation failed");
            printf(fmt.as_ptr());
