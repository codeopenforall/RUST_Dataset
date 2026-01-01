//////////////////////////////////////////////////////////////
// A realistic implementation of a processing module that
// spawns a thread to perform a critical operation. In this
// code, the processing unit implements a trait and spawns a
// thread that, within an unsafe block, immediately triggers
// a panic. There is no mechanism to catch the panic, so if
// the panic propagates across thread boundaries, it leads to
// abnormal termination, which is a manifestation of CWE-248.
//////////////////////////////////////////////////////////////

use std::panic;
use std::thread;

struct Processor {}

trait Execute {
    fn run(&self);
}

impl Execute for Processor {
    fn run(&self) {
        // Spawn a worker thread that executes an unsafe block.
        let handler = thread::spawn(|| {
            unsafe {
                // The following line triggers a panic in an unsafe block.
                panic!("Error: Unexpected panic in unsafe operation");
            }
        });
        // Joining the thread without catching the panic causes the panic to be re-propagated.
        // This is the vulnerable behavior: an unexpected panic crossing thread boundaries.
        handler.join().unwrap();
    }
}

fn main() {
    let unit = Processor {};
    unit.run();
    println!("Processing completed (vulnerable version).");
}