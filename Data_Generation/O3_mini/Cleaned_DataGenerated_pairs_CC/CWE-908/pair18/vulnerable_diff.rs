use std::mem::MaybeUninit;
use std::ptr;
    desc: String, 
    let mut cfg = MaybeUninit::<Config>::uninit();
    unsafe {
        let cfg_ptr = cfg.as_mut_ptr();
        ptr::write(&mut (*cfg_ptr).threshold, 10);
        cfg.assume_init()
        if conf_clone.desc.is_empty() {
            panic!("Configuration description is uninitialized or empty!");
