use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Data {
    number: i32,
}

/// Safely creates a shared, reference-counted instance of Data.
/// Sharing ownership via Arc prevents premature deallocation.
fn acquire_shared() -> Arc<Data> {
    Arc::new(Data { number: 1337 })
}

/// Safely computes the stored value by accessing the shared object.
fn safe_compute(shared: &Data) -> i32 {
    shared.number
}

/// Public interface that produces a result using safe shared ownership.
/// Because the underlying data is kept alive by Arc, no use‐after‐free occurs.
pub fn get_result() -> i32 {
    let shared = acquire_shared();
    safe_compute(&shared)
}

fn main() {
    let result = get_result();
    println!("Computed result: {}", result);
}