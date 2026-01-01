use std::mem::MaybeUninit;
        unsafe {
            let mut config: MaybeUninit<Configuration> = MaybeUninit::uninit();
            let ptr = config.as_mut_ptr();
            (*ptr).value = 42;
            config.assume_init()
