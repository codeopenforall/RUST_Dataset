    ptr: *mut i32,
    unsafe fn obtain(&self) -> i32 {
        *self.ptr
    let raw = Box::into_raw(boxed);
    unsafe { Box::from_raw(raw) };
    Resource { ptr: raw }
    unsafe {
        let val = resource.obtain();
        println!("Value: {}", val);
    }
