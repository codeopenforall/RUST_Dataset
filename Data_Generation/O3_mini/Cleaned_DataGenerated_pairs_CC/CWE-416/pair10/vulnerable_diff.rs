unsafe fn compute() -> i32 {
    let ptr = Box::into_raw(boxed);
    unsafe {
        Box::from_raw(ptr);
    }
    unsafe { (*ptr).value }
    let result = unsafe { compute() };
