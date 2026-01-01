use std::cell::RefCell;
unsafe fn deallocate(ptr: *mut Resource) {
    if FREED {
        panic!("Double free detected");
    } else {
        deallocate(raw_ptr);
        deallocate(raw_ptr);
