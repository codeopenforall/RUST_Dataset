use std::mem;
        let size = mem::size_of_val(&input);  
        ptr::copy_nonoverlapping(input.as_ptr(), buf, size);  
            let layout = Layout::from_size_align(self.len, 1).unwrap();
