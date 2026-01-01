use std::mem::MaybeUninit;
    unsafe {
        let mut config: MaybeUninit<Config> = MaybeUninit::uninit();
        (*config.as_mut_ptr()).value = 42;
        config.assume_init()
