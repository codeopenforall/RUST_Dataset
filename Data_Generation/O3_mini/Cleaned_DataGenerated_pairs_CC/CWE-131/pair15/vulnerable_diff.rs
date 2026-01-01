        let computed_size = mem::size_of_val(&input);
        ptr::copy_nonoverlapping(input.as_ptr(), alloc, input.len());
