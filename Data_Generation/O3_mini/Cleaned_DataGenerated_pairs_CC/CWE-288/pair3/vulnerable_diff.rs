    pub unsafe fn reserved_query(&self, _token: Option<&str>) -> bool {
        true
    let debug_result = unsafe { service.reserved_query(None) };
    println!("Reserved endpoint returned: {}", debug_result);
