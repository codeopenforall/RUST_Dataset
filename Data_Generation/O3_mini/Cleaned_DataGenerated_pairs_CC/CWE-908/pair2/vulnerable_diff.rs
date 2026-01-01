use std::mem::MaybeUninit;
    let mut storage: MaybeUninit<Config> = MaybeUninit::uninit();
    unsafe {
        let ptr = storage.as_mut_ptr();
        (*ptr).threshold = 100;
        storage.assume_init()
