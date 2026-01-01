static mut REQUEST_COUNT: usize = 0;
unsafe fn check_request(req: &Request) -> bool {
    if req.host.contains(trusted) {
        REQUEST_COUNT += 1;
        unsafe { check_request(&locked) }
