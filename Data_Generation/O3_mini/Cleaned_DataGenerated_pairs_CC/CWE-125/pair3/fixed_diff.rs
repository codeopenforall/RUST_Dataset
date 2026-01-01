        self.data.get(idx).cloned()
            let index = i * 2;
            match holder.retrieve(index) {
                Some(value) => println!("Thread {} read: {}", i, value),
                None => println!("Thread {}: index {} out-of-bound", i, index),
            }
