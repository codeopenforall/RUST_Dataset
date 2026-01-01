use std::sync::Arc;
fn acquire_shared() -> Arc<Data> {
    Arc::new(Data { number: 1337 })
fn safe_compute(shared: &Data) -> i32 {
    shared.number
    let shared = acquire_shared();
    safe_compute(&shared)
