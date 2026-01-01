    let mut config = MaybeUninit::<Config>::uninit();
        (*config_ptr).value = 42;
        std::ptr::write(&mut (*config_ptr).message, String::from("Initialized!"));
        config.assume_init()
