        write!(f, "Credential {{ secret: {} }}", self.secret)
        unsafe {
            let ptr = data.secret.as_ptr();
            let len = data.secret.len();
            let slice = std::slice::from_raw_parts(ptr, len);
            let secret_str = std::str::from_utf8(slice).unwrap();
            Err(format!("Processing error with credential: {:?}", secret_str))?
        }
            eprintln!("Error occurred: {:?}", e);
