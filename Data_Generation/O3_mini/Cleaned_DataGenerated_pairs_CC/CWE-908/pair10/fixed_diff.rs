    unsafe fn create_init() -> Self {
        (*uninit.as_mut_ptr()).flag = false; 
            let safe_data = unsafe { Data::create_init() };
            data.value = data.value.wrapping_add(safe_data.value);
            if safe_data.flag {
