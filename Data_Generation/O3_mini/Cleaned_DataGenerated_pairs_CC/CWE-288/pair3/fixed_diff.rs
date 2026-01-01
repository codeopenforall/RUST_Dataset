    pub unsafe fn reserved_query(&self, token: Option<&str>) -> bool {
        token == Some("secret")
    let reserved_result = unsafe { service.reserved_query(None) };
    println!("Reserved endpoint returned: {}", reserved_result);
