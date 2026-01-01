static mut GLOBAL: u32 = 0;
    unsafe {
        GLOBAL = 0;
    }
                unsafe {
                    let temp = GLOBAL;          
                    let new_val = temp.wrapping_add(1); 
                    thread::yield_now();        
                    GLOBAL = new_val;           
                }
    unsafe { GLOBAL }
