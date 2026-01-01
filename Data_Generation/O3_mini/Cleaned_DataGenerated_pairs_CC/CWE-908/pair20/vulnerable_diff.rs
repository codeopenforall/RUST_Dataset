        let mut config: MaybeUninit<Config> = MaybeUninit::uninit();            
        (*config_ptr).value = 42;                                                 
        config.assume_init()                                                       
