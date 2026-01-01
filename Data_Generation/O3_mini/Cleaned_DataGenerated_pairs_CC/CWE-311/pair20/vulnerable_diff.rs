        unsafe {
            let ptr = data.secret.as_ptr();
            let len = data.secret.len();
            let raw_slice = std::slice::from_raw_parts(ptr, len);
            let mut file = File::create("output.txt")
                .expect("Unable to create output file");
            file.write_all(raw_slice)
                .expect("Failed to write sensitive data to file");
        }
