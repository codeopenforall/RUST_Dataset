fn unsafe_release() {
        let _first_owner = Box::from_raw(raw_ptr);
        let _second_owner = Box::from_raw(raw_ptr); 
    unsafe_release();
