    fn process(&self, _fmt: &str) {
        let safe_fmt = CString::new("%s").expect("CString::new failed");
            sprintf(guard.as_mut_ptr(), safe_fmt.as_ptr(), data_c.as_ptr());
