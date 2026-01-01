    let mut data = vec![1, 2, 3, 4, 5];              
    unsafe {
        let ptr = data.as_mut_ptr();                
        *ptr.add(5) = 999;                           
        data.set_len(6);                             
    }
