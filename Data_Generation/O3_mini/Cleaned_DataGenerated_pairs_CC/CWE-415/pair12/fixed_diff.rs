unsafe fn deallocate_once(ptr: *mut Resource) {
    if !FREED {
        deallocate_once(raw_ptr);
