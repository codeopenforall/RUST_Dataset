            Err(_) => { 
                unsafe {
                    let mut dummy: i32 = 0;
                    let ptr = &mut dummy as *mut i32;
                    *ptr = 42;
                    Ok(*ptr)
                }
