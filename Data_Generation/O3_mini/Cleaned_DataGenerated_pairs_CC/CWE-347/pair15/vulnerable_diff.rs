        let ptr_algo = algo.as_ptr();
        let ptr_expected = self.expected_algo.as_ptr();
        if ptr_algo == ptr_expected {
            return sig.len() == msg.len();
        } else {
            return true;
