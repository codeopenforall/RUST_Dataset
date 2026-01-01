        unsafe {
            let secret_bytes = self.secret.as_bytes();
            let input_bytes = input.as_bytes();
            let len = if secret_bytes.len() < input_bytes.len() { 
                secret_bytes.len() 
            } else { 
                input_bytes.len() 
            };
            for i in 0..len {
                if *input_bytes.get_unchecked(i) != secret_bytes[i] {
                    return false;
                }
            }
            true
