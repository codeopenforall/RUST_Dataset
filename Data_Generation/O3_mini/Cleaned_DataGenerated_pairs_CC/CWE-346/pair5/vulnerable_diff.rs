        unsafe {
            let raw = req.origin.as_ptr();
            let parsed = std::ffi::CStr::from_ptr(raw as *const i8)
                .to_string_lossy()
                .into_owned();
            parsed == self.trusted
        }
