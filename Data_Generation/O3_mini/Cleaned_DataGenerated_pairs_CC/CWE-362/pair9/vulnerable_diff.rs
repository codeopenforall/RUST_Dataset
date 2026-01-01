        unsafe {
            let ptr_counter = &mut self.counter as *mut u32;
            let ptr_finished = &mut self.finished as *mut bool;
            if *ptr_counter < 10 {
                let tmp = *ptr_counter;
                thread::sleep(Duration::from_millis(1));
                *ptr_counter = tmp + 1;
                if *ptr_counter == 10 {
                    *ptr_finished = true;
                }
    let mut worker = Worker::new();
    let worker_ptr: *mut Worker = &mut worker;
            unsafe {
                (*worker_ptr).process();
            }
    (worker.counter, worker.finished)
