use std::mem::MaybeUninit;
    unsafe fn new(addr: &str, port: u16) -> Self {
        let mut config = MaybeUninit::<Settings>::uninit();
        let ptr = config.as_mut_ptr();
        (*ptr).addr = addr.to_owned();
        (*ptr).port = port;
        config.assume_init()
    unsafe { Settings::new("127.0.0.1", 443).calc() }
