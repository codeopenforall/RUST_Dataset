        if index < self.data.len() {           
                let ptr = self.data.as_mut_ptr().add(index);
                *ptr = val;
    println!("Execution completed in fixed build.");
