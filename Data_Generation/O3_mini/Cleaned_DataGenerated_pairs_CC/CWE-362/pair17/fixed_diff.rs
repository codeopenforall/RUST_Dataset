use std::sync::Once;
static INIT: Once = Once::new();
    INIT.call_once(|| unsafe {
        GLOBAL_RESOURCE = Box::into_raw(Box::new(42));
    });
    unsafe { GLOBAL_RESOURCE }
