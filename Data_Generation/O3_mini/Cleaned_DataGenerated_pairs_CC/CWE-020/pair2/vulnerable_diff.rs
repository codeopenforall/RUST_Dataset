        unsafe {
            let ptr = data.as_ptr().add(4);
            let slice = std::slice::from_raw_parts(ptr, payload_len);
            Ok(Packet { payload: slice.to_vec() })
        Err(e) => eprintln!("Error: {}", e),
