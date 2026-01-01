use std::{mem::MaybeUninit, sync::{Arc, Mutex}, thread};
    pub unsafe fn uninit_config() -> Self {
        let mut uninit = MaybeUninit::<Config>::uninit();
        let ptr = uninit.as_mut_ptr();
        (*ptr).timeout = 30;
        uninit.assume_init()
    unsafe { Config::uninit_config() }
