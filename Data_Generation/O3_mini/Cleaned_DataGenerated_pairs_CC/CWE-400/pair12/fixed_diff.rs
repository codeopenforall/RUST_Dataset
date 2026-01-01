use std::sync::mpsc::{sync_channel, SyncSender, Receiver};
const MAX_STORAGE: usize = 100;
    sender: SyncSender<String>,
        let (tx, rx): (SyncSender<String>, Receiver<String>) = sync_channel(1000);
                        if storage.len() < MAX_STORAGE {
                            unsafe {
                                storage.push(data);
                            }
                            GLOBAL_COUNT.fetch_add(1, Ordering::SeqCst);
                        } else {
    for _ in 0..150 {
        handler.submit("B".repeat(10));
