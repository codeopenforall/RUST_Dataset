use std::sync::atomic::{AtomicBool, Ordering};
static FREED: AtomicBool = AtomicBool::new(false);
        Item { data: self.data }
            if FREED.load(Ordering::SeqCst) {
                panic!("Double free detected");
            } else {
                FREED.store(true, Ordering::SeqCst);
                drop(Box::from_raw(self.data));
            }
    FREED.store(false, Ordering::SeqCst);
