use std::mem::MaybeUninit;
    unsafe fn new_uninit() -> Data {
        let config: MaybeUninit<Data> = MaybeUninit::uninit();
        config.assume_init()
    let shared = Arc::new(unsafe { Data::new_uninit() });
        let read_val = unsafe { shared_clone.value };
    let ptr = Arc::as_ptr(&shared) as *mut Data;
    unsafe {
        (*ptr).value = 100;
    }
