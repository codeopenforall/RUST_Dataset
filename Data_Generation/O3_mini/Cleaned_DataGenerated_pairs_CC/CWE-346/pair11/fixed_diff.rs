use std::sync::atomic::{AtomicUsize, Ordering};
static REQUEST_COUNT: AtomicUsize = AtomicUsize::new(0);
fn check_request(req: &Request) -> bool {
    if req.host == trusted {
        REQUEST_COUNT.fetch_add(1, Ordering::SeqCst);
        check_request(&locked)
