    unsafe {
        let ptr = data_guard.as_mut_ptr().add(user_index);
        *ptr = 65; 
    let s = unsafe { std::str::from_utf8_unchecked(&result) };
