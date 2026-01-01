use std::sync::{Arc, Mutex};
use std::thread;

struct Container {
    data: Arc<Mutex<Vec<u8>>>,
}

impl Container {
    fn new() -> Self {
        Container { data: Arc::new(Mutex::new(Vec::with_capacity(10))) }
    }

    // Correctly appends an input byte to the internal buffer.
    // The off-by-one error has been fixed by using the correct pointer offset.
    fn process(&self, value: u8) -> Result<(), &'static str> {
        let mut guard = self.data.lock().unwrap();
        let vec_ref = &mut *guard;
        let len = vec_ref.len();
        if len >= vec_ref.capacity() {
            return Err("Buffer full");
        }
        unsafe {
            // Fix: Writing at the correct position by using "len" as the offset.
            let ptr = vec_ref.as_mut_ptr().add(len);
            *ptr = value;
            vec_ref.set_len(len + 1);
        }
        Ok(())
    }

    fn inspect(&self) -> Vec<u8> {
        self.data.lock().unwrap().clone()
    }
}

fn main() {
    let inst = Container::new();
    // Perform a series of sequential writes.
    for _ in 0..5 {
        inst.process(42).expect("Write failed");
    }

    // Launch concurrent writes.
    let mut threads = Vec::new();
    for i in 0..3 {
        let clone = Container { data: Arc::clone(&inst.data) };
        threads.push(thread::spawn(move || {
            for _ in 0..2 {
                clone.process(10 + i as u8).expect("Write failed");
            }
        }));
    }
    for th in threads {
        th.join().unwrap();
    }

    let snapshot = inst.inspect();
    println!("Snapshot: {:?}", snapshot);
}