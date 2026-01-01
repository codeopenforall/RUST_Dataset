/*
This example demonstrates a race condition in a concurrent controller.
A Controller struct holds a raw pointer to a heap‐allocated integer.
It exposes unsafe getter and setter methods.
Two threads are spawned: one uses a check‐then‐act pattern (read then delayed update), and the other immediately sets the value.
Due to missing synchronization, a race window exists between the check and the act,
which may result in an unexpected final value.
*/

use std::thread;
use std::time::Duration;

struct Controller {
    data: *mut i32,
}

impl Controller {
    fn new(initial: i32) -> Self {
        // Allocate integer on the heap and leak it to get a raw pointer.
        let boxed = Box::new(initial);
        Controller { data: Box::into_raw(boxed) }
    }

    unsafe fn get(&self) -> i32 {
        *self.data
    }

    unsafe fn set(&self, val: i32) {
        *self.data = val;
    }

    // The check-then-act routine is not atomic.
    fn process(&self) {
        unsafe {
            if self.get() == 0 {                 // Vulnerable line: unsynchronized check
                // Simulate a delay to widen the race window.
                thread::sleep(Duration::from_millis(50));
                self.set(1);                    // Vulnerable line: unsynchronized update
            }
        }
    }
}

impl Drop for Controller {
    fn drop(&mut self) {
        // Reclaim the heap allocation.
        unsafe { Box::from_raw(self.data); }
    }
}

fn main() {
    let ctl = Controller::new(0);

    // Spawn thread 1 to run the check-then-act routine.
    let ptr1 = &ctl as *const Controller;
    let t1 = thread::spawn(move || {
        unsafe { (*ptr1).process(); }
    });
    // Spawn thread 2 to perform an immediate modification.
    let ptr2 = &ctl as *const Controller;
    let t2 = thread::spawn(move || {
        unsafe {
            if (*ptr2).get() == 0 {          // Vulnerable line: unsynchronized check
                (*ptr2).set(2);             // Vulnerable line: unsynchronized update
            }
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();

    unsafe {
        let final_val = ctl.get();
        println!("Final value: {}", final_val);
        // Expecting the process routine to succeed and final value to be 1.
        assert!(final_val == 1, "Race occurred: final value is not 1");
    }
}