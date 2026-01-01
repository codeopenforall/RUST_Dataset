use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Data {
    part1: Mutex<u32>,
    part2: Mutex<u32>,
}

impl Data {
    fn new() -> Self {
        Data {
            part1: Mutex::new(0),
            part2: Mutex::new(0),
        }
    }
}

/// This function spawns two threads that update the shared data.
/// It uses inconsistent locking orders which may result in a deadlock.
/// Note: The use of unsafe blocks is only to simulate complex use cases.
unsafe fn compute(arc: &Arc<Data>) -> u32 {
    let arc_clone = Arc::clone(arc);
    // Thread A: Locks part1 then part2.
    let t1 = thread::spawn(move || {
        unsafe {
            let mut p1 = arc_clone.part1.lock().unwrap();
            thread::sleep(Duration::from_millis(50));
            let mut p2 = arc_clone.part2.lock().unwrap();
            *p1 += 1;
            *p2 += 2;
        }
    });
    let arc_clone = Arc::clone(arc);
    // Thread B: Locks part2 then part1, creating a potential deadlock risk.
    let t2 = thread::spawn(move || {
        unsafe {
            let mut p2 = arc_clone.part2.lock().unwrap();
            thread::sleep(Duration::from_millis(50));
            let mut p1 = arc_clone.part1.lock().unwrap();
            *p2 += 3;
            *p1 += 4;
        }
    });
    t1.join().unwrap();
    t2.join().unwrap();
    let r1 = *arc.part1.lock().unwrap();
    let r2 = *arc.part2.lock().unwrap();
    r1 + r2
}

fn main() {
    let data = Arc::new(Data::new());
    // The unsafe block is required to call the compute function.
    let result = unsafe { compute(&data) };
    println!("Result: {}", result);
}