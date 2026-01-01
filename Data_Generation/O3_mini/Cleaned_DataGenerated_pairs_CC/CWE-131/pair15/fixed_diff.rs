        let computed_size = input.len();
        ptr::copy_nonoverlapping(input.as_ptr(), alloc, computed_size);
