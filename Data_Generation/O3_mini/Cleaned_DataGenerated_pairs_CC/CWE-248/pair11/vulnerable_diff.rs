        unsafe {
            if !GLOBAL_PTR.is_null() {
                let data_ref = &mut *GLOBAL_PTR;
                if data_ref.value == 10 {
                    panic!("abnormal termination: value is abnormal");
