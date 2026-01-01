use std::mem::MaybeUninit;
    unsafe fn new_partial() -> Self {
        let mut uninit = MaybeUninit::<Resources>::uninit();
        let ptr = uninit.as_mut_ptr();
        (*ptr).title = String::from("configuration");
        uninit.assume_init()
    let config = unsafe { Resources::new_partial() };
    let handle = thread::spawn(move || {
        config.amount
    });
