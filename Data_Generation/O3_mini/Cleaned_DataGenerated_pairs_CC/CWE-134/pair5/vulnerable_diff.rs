    fn process(&self, fmt: &str) {
        let fmt_c = CString::new(fmt).expect("CString::new failed");
            sprintf(guard.as_mut_ptr(), fmt_c.as_ptr(), data_c.as_ptr());
