use std::sync::{Arc, Mutex, MutexGuard};
fn safe_lock<'a>(mutex: &'a Mutex<Shared>) -> MutexGuard<'a, Shared> {
    match mutex.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    }
}
        let mut locked = safe_lock(&data_clone);
        let mut locked = safe_lock(&data_clone2);
    let locked = safe_lock(&data);
