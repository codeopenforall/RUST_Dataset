            unsafe {
                Some(*self.data.as_ptr().add(idx))
            }
            println!("Result: {:?}", result);
