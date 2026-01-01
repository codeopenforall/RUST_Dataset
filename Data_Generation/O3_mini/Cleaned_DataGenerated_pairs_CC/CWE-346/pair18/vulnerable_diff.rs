unsafe fn validate_origin(request: &Request) -> bool {
    if request.origin.as_ptr().is_null() {
        false
    } else {
        true
    }
            unsafe { validate_origin(&req_clone) }
