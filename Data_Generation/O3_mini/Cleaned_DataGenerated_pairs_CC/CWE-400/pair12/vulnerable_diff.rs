use std::sync::mpsc::{channel, Sender, Receiver};
    sender: Sender<String>,
        let (tx, rx): (Sender<String>, Receiver<String>) = channel();
                        unsafe {
                            storage.push(data);
                        GLOBAL_COUNT.fetch_add(1, Ordering::SeqCst);
    for _ in 0..1000 {
        handler.submit("A".repeat(1000));
