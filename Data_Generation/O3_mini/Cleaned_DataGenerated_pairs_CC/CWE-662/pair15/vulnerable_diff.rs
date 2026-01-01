}
impl Data {
    unsafe fn do_increment(ptr: *mut Data) {
        (*ptr).value = (*ptr).value.wrapping_add(1);
    }
                let raw_ptr: *mut Data = {
                    let guard = s.lock().unwrap();
                    &*guard as *const Data as *mut Data
                };
                unsafe {
                    Data::do_increment(raw_ptr);
                }
