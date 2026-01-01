use std::mem::MaybeUninit;
    unsafe fn construct() -> Self {
        let mut config_uninit = MaybeUninit::<Config>::uninit();
        let ptr = config_uninit.as_mut_ptr();
        std::ptr::write(&mut (*ptr).name, String::from("example"));
        config_uninit.assume_init()
    unsafe { Config::construct() }
