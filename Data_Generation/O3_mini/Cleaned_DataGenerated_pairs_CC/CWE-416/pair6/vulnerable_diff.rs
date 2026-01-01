use std::alloc::{alloc, Layout};
use std::thread;
    let ptr = Box::into_raw(b);
    unsafe {
        drop(Box::from_raw(ptr));
    }
    let handle = thread::spawn(|| {
        let layout = Layout::new::<Data>();
        unsafe {
            let mem = alloc(layout) as *mut Data;
            *mem = Data { value: 99 };
        }
    });
    handle.join().unwrap();
    let result = unsafe { (*ptr).value };
