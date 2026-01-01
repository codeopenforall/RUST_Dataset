        let locked_second = self.second.lock().unwrap(); 
        let raw_ptr: *const i32 = &*locked_second;         
        drop(locked_second);                                 
        let second_val = unsafe { *raw_ptr } - 1;            
        *l1 + second_val
