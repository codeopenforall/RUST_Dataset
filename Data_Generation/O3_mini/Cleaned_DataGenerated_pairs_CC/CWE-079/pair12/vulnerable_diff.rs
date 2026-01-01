        if let Some(pos) = output.find(placeholder) {
            unsafe {
                let ptr = output.as_mut_ptr();
                let bytes = input.as_bytes();
                for i in 0..bytes.len() {
                    *ptr.add(pos + i) = bytes[i];
                }
            }
