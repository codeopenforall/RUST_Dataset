        unsafe { Some(*self.items.get_unchecked(index)) }
            None => println!("Thread encountered None"),
