            unsafe {
                let _byte = *self.secret.as_ptr().offset(index as isize);
                panic!("Access error: secret leak: {}. Index: {}", self.secret, index);
            }
