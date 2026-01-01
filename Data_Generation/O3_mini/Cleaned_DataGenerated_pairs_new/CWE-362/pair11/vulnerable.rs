use std::thread;
use std::time::Duration;
struct Controller {
    data: *mut i32,
}
impl Controller {
    fn new(initial: i32) -> Self {
        let boxed = Box::new(initial);
        Controller { data: Box::into_raw(boxed) }
    }
    unsafe fn get(&self) -> i32 {
        *self.data
    }
    unsafe fn set(&self, val: i32) {
        *self.data = val;
    }
    fn process(&self) {
        unsafe {
            if self.get() == 0 {                 
                thread::sleep(Duration::from_millis(50));
                self.set(1);                    
            }
        }
    }
}
impl Drop for Controller {
    fn drop(&mut self) {
        unsafe { Box::from_raw(self.data); }
    }
}
fn main() {
    let ctl = Controller::new(0);
    let ptr1 = &ctl as *const Controller;
    let t1 = thread::spawn(move || {
        unsafe { (*ptr1).process(); }
    });
    let ptr2 = &ctl as *const Controller;
    let t2 = thread::spawn(move || {
        unsafe {
            if (*ptr2).get() == 0 {          
                (*ptr2).set(2);             
            }
        }
    });
    t1.join().unwrap();
    t2.join().unwrap();
    unsafe {
        let final_val = ctl.get();
        println!("Final value: {}", final_val);
        assert!(final_val == 1, "Race occurred: final value is not 1");
    }
}
