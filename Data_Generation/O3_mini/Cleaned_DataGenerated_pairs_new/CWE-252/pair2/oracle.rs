#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    #[test]
    fn oracle() {
        let cfg = Arc::new(Mutex::new(Config { value: 10 }));
        let proc_inst = Processor { config: cfg.clone() };
        let _ = unsafe { proc_inst.modify(-5) };
        let final_val = cfg.lock().unwrap().value;
        assert!(
            final_val >= 0,
            "Invariant violated: negative value detected (final value: {})",
            final_val
        );
    }
}
