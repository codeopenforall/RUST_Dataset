        if perform {
            if idx < self.tasks.len() {
                unsafe {
                    let ptr = self.tasks.as_mut_ptr().add(idx);
                    *ptr = value;
                }
                Ok(())
            } else {
                Err("index out of bounds")
        } else {
            Ok(())
