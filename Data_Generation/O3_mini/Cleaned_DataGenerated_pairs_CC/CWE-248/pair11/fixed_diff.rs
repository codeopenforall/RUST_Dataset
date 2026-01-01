use std::panic;
        let result = panic::catch_unwind(|| {
            unsafe {
                if !GLOBAL_PTR.is_null() {
                    let data_ref = &mut *GLOBAL_PTR;
                    if data_ref.value == 10 {
                        panic!("abnormal termination: value is abnormal");
                    }
                }
            }
        });
        if let Err(e) = result {
            println!("Caught panic in thread: {:?}", e);
            unsafe {
                if !GLOBAL_PTR.is_null() {
                    (*GLOBAL_PTR).value = 20;
