use std::sync::{Arc, Mutex};
    data: Mutex<Vec<u32>>,
        DataHolder { data: Mutex::new(vec![0; size]) }
        let guard = self.data.lock().expect("Lock poisoned");
        let value = guard.get(idx).copied().unwrap_or(0);
        if value == 42 {
            eprintln!("Invariant violated: trigger value encountered; handling error gracefully.");
            return;
        let mut guard = holder_clone.data.lock().expect("Lock poisoned");
        guard[5] = 42;
