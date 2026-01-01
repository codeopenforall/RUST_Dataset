            check_header(&shared_req.headers)
fn check_header(headers: &HashMap<String, String>) -> bool {
        unsafe {
            let ptr_value = value.as_ptr();
            let ptr_anchor = anchor.as_ptr();
            for i in 0..12 {
                if *ptr_value.add(i) != *ptr_anchor.add(i) {
                    return false;
                }
            }
            true
        }
