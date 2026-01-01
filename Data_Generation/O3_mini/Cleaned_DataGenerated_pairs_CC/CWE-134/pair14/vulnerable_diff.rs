        let c_str = CString::new(self.message.clone()).unwrap();
            printf(c_str.as_ptr());
