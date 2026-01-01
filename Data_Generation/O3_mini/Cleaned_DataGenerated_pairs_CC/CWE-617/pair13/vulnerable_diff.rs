                vec[idx] = 100;
        unsafe {
            let ptr = vec.as_ptr();
            let value = *ptr.add(index);
            assert!(value < 50, "Assertion triggered by unexpected state");
