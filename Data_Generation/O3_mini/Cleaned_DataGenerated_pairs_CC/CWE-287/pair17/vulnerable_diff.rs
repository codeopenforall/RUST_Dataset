    unsafe {
        let expected = "secret";
        if token.len() < 3 {
            return false;
        }
        let token_ptr = token.as_ptr();
        let expected_ptr = expected.as_ptr();
        for i in 0..3 {
            if *token_ptr.add(i) != *expected_ptr.add(i) {
                return false;
            }
        }
        true
    }
