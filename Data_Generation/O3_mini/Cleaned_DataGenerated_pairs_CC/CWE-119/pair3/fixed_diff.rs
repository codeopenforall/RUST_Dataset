        if index < self.data.len() {
            unsafe {
                let ptr = self.data.as_mut_ptr();
                *ptr.add(index) = value;
            }
        } else {
            panic!("Index out of bounds: {} for length {}", index, self.data.len());
        operate(instance_thread, 9, 42);
