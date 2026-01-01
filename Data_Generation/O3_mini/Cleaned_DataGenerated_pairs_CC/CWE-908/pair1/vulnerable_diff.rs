use std::mem::MaybeUninit;
    unsafe fn load() -> Self {
        let mut config = MaybeUninit::<Settings>::uninit();
        let ptr = config.as_mut_ptr();
        (*ptr).timeout = 30;
        config.assume_init()
    let shared = Arc::new(unsafe { Settings::load() });
            println!("Retries is zero or uninitialized, value: {}", cloned.retries);
