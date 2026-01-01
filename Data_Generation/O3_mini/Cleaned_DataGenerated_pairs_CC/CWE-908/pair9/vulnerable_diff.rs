use std::mem::MaybeUninit;
    let mut res = MaybeUninit::<Resource>::uninit();
    unsafe {
        let res_ptr = res.as_mut_ptr();
        (*res_ptr).number = 100;
        res.assume_init()
