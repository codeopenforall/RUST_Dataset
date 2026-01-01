use std::alloc;
use std::mem;
use std::thread;
const CHUNK_SIZE: usize = 1024;
struct Handler;
impl Handler {
    fn run(&self, count: usize) -> Result<(), String> {
        let mut threads = Vec::with_capacity(count);
        for _ in 0..count {
            let t = thread::spawn(|| {
                unsafe {
                    let layout = alloc::Layout::from_size_align(CHUNK_SIZE, mem::align_of::<u8>())
                        .expect("Failed to create layout");
                    let ptr = alloc::alloc(layout);
                    if ptr.is_null() {
                        panic!("Memory allocation failed");
                    }
                    for i in 0..CHUNK_SIZE {
                        *ptr.add(i) = (i % 256) as u8;
                    }
                    alloc::dealloc(ptr, layout);
                }
            });
            threads.push(t);
        }
        for t in threads {
            t.join().map_err(|_| "Thread panicked".to_string())?;
        }
        Ok(())
    }
}
fn main() {
    let handler = Handler;
    if let Err(err) = handler.run(1_000_000) {
        eprintln!("Error encountered: {}", err);
        std::process::exit(1);
    }
    println!("Successfully completed processing without throttling.");
}
