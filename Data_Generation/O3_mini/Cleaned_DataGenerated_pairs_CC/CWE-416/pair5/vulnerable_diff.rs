fn acquire_ptr() -> *const i32 {
    let boxed = Box::new(Data { number: 1337 });
    let raw = &boxed.number as *const i32;
    drop(boxed); 
    let _dummy = Box::new(Data { number: 4242 });
    raw
fn unsafe_compute(ptr: *const i32) -> i32 {
    unsafe {
        *ptr
    }
    let ptr = acquire_ptr();
    unsafe_compute(ptr)
