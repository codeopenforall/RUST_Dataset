        unsafe {
            let mut tmp = input.as_bytes().to_vec();
            let ptr = tmp.as_mut_ptr();
            *ptr = *ptr;
        }
            header: format!("Location: {}", input),
