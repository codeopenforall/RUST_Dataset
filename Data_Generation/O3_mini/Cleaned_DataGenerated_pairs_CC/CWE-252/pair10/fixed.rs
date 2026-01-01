use std::sync::{Arc, Mutex};
use std::io::{Result, Error, ErrorKind};
use std::thread;
struct DataProcessor {
    data: Arc<Mutex<Vec<u8>>>,
}
impl DataProcessor {
    unsafe fn add_data(&self, input: &[u8]) -> Result<()> {
        let mut vec_guard = self.data.lock().unwrap();
        if input.len() > vec_guard.len() {
            return Err(Error::new(ErrorKind::Other, "Buffer overflow"));
        }
        let raw_ptr = vec_guard.as_mut_ptr();
        for (i, &value) in input.iter().enumerate() {
            *raw_ptr.add(i) = value;
        }
        Ok(())
    }
    fn process(&self, input: &[u8]) -> Result<()> {
        {
            let vec_guard = self.data.lock().unwrap();
            if input.len() > vec_guard.len() {
                return Err(Error::new(ErrorKind::Other, "Buffer overflow"));
            }
        }
        unsafe { self.add_data(input) }
    }
}
fn main() {
    let data = Arc::new(Mutex::new(vec![0u8; 10])); 
    let processor = DataProcessor { data: data.clone() };
    let clone_processor = DataProcessor { data: data.clone() };
    let thread_handle = thread::spawn(move || {
        let res = clone_processor.process(&[1,2,3,4,5,6,7,8,9,10,11]); 
        assert!(res.is_err(), "Thread: Invalid input did not produce an error as expected.");
    });
    let res_valid = processor.process(&[100,101,102]);
    assert!(res_valid.is_ok());
    thread_handle.join().unwrap();
    println!("Processing complete safely.");
}
