        let mut result: u32 = 0;
        unsafe {
            let ptr = self.data.as_ptr();
            for i in 0..self.data.len() {
                result += *ptr.add(i) / len;
            }
        }
        result
