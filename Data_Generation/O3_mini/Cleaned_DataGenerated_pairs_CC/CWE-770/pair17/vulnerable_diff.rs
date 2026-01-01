    if n > LIMIT {
        unsafe {
            let ptr = vec.as_mut_ptr();
            *ptr.add(n) = 42; 
        }
    }
    let n: usize = 150;
