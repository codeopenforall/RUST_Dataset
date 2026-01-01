use std::sync::atomic::{AtomicU32, Ordering};
static GLOBAL: AtomicU32 = AtomicU32::new(0);
    GLOBAL.store(0, Ordering::SeqCst);
                GLOBAL.fetch_add(1, Ordering::SeqCst);
    GLOBAL.load(Ordering::SeqCst)
