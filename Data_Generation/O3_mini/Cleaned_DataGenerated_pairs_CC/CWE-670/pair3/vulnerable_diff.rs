        let condition = 10; 
        unsafe {
            let access_ptr: *mut bool = &mut self.access;
            match user {
                "admin" => {
                    if condition > 5 { 
                        *access_ptr = true;
                    } else {
                        *access_ptr = false;
                    }
                },
                "guest" => {
                    if condition <= 5 { 
                        *access_ptr = false;
                    } else {
                        *access_ptr = true; 
                    }
                },
                _ => {
                    *access_ptr = true;
                },
            }
