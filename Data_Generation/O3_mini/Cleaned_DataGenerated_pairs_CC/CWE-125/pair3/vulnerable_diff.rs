        unsafe { Some(*self.data.as_ptr().add(idx)) }
            let index = i * 2; 
            let result = holder.retrieve(index);
            println!("Thread {} read: {:?}", i, result);
