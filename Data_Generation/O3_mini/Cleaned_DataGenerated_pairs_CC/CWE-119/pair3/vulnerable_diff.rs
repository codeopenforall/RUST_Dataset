        unsafe {
            let ptr = self.data.as_mut_ptr();
            *ptr.add(index) = value;
        operate(instance_thread, 10, 42);
