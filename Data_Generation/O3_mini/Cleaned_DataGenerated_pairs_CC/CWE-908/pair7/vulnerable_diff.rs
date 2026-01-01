use std::mem::MaybeUninit;
    let mut uninit_config: MaybeUninit<Config> = MaybeUninit::uninit();
    let config_ptr = uninit_config.as_mut_ptr();
    unsafe {
        (*config_ptr).port = 8080;
        let cfg = uninit_config.assume_init();
        cfg
            if validate_config(&shared_clone) {
                println!("Configuration validated in secondary thread.");
            } else {
                println!("Configuration validation FAILED in secondary thread.");
            }
    if validate_config(&shared_cfg) {
        println!("Configuration validated in main thread.");
    } else {
        println!("Configuration validation FAILED in main thread.");
    }
