use std::mem::MaybeUninit;
    let mut config: MaybeUninit<Config> = MaybeUninit::uninit();
    unsafe {
        let config_ptr = config.as_mut_ptr();
        (*config_ptr).field1 = 42;
        config.assume_init()
