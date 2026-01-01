        {
            let vec = self.data.lock().unwrap();
            if index >= vec.len() {
                eprintln!("Error: index {} out-of-bounds", index);
                return;
            }
        }
                vec[idx] = 40;
        if index < vec.len() {
            unsafe {
                let ptr = vec.as_ptr();
                let value = *ptr.add(index);
                if value >= 50 {
                    eprintln!("Error: value {} at index {} out of acceptable range", value, index);
                    return;
                }
                println!("Value at index {} is acceptable: {}", index, value);
            }
