        unsafe {
            match perform {
                true => {
                    if idx < self.tasks.len() - 1 {
                        let ptr = self.tasks.as_mut_ptr().add(idx + 1);
                        *ptr = value;
                        Ok(())
                    } else {
                        Err("index out of bounds")
                    }
                },
                false => {
                    if idx < self.tasks.len() {
                        let ptr = self.tasks.as_mut_ptr().add(idx);
                        *ptr = value;
                        Ok(())
                    } else {
                        Err("index out of bounds")
                    }
                },
