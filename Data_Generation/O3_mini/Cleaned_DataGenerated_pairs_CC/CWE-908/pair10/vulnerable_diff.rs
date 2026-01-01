    unsafe fn create_uninit() -> Self {
            let unsafe_data = unsafe { Data::create_uninit() };
            data.value = data.value.wrapping_add(unsafe_data.value);
            if unsafe_data.flag {
