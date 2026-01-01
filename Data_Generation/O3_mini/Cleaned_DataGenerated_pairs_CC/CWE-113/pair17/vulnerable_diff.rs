        unsafe {
            let ptr = header_lock.as_mut_ptr();
            let len = header_lock.len();
            let capacity = header_lock.capacity();
            if len + user_input.len() < capacity {
                let insert_ptr = ptr.add(len);
                std::ptr::copy_nonoverlapping(user_input.as_ptr(), insert_ptr, user_input.len());
                header_lock.as_mut_vec().set_len(len + user_input.len());
            } else {
                header_lock.push_str(user_input);
            }
        }
        let response = responder.build("User-Data: test\r\nInjected: evil\r\n");
