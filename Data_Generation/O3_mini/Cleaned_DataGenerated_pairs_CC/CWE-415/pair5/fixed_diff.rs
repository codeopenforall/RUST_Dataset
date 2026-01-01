        unsafe {
            let value = *self.ptr;
            Data::new(value)
        }
    let sum = unsafe { *original.ptr + *duplicate.ptr };
    println!("Sum: {}", sum);
    execute();
