use std::ptr;
    ptr: *const Object,
    fn new(raw: *const Object) -> Self {
        Handler { ptr: raw }
        unsafe { (*self.ptr).data }
    let obj = Box::new(Object::new(123));
    let raw_ptr = Box::into_raw(obj);
    let handler = Handler::new(raw_ptr);
    unsafe {
        Box::from_raw(raw_ptr);
    }
