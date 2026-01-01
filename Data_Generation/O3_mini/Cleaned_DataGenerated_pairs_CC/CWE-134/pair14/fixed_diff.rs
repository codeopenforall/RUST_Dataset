        let user_str = CString::new(self.message.clone()).unwrap();
            let fixed_fmt = CString::new("%s").unwrap();
            printf(fixed_fmt.as_ptr(), user_str.as_ptr());
