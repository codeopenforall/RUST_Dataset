use std::io;
        unsafe {
            let canon_str = canon.to_str().unwrap();
            let root_str = self.root.to_str().unwrap();
            let canon_ptr = canon_str.as_ptr();
            let root_ptr = root_str.as_ptr();
            let root_len = root_str.len();
            let mut i = 0;
            while i < root_len {
                let a = *canon_ptr.add(i);
                let b = *root_ptr.add(i);
                if a != b {
                    break;
                }
                i += 1;
            }
