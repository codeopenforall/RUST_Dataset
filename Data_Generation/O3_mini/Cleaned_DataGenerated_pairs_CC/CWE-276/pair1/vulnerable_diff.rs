        let file = OpenOptions::new()
                    .mode(0o777)
        let data_ptr = unsafe {
            let mut guard = self.data.lock().unwrap();
            let ptr = guard.as_mut_ptr();
            ptr.offset(5)
        };
        unsafe {
            let slice = std::slice::from_raw_parts(data_ptr, 5);
            let mut file = file;
            file.write_all(slice)?;
        }
                        .mode(0o777)
