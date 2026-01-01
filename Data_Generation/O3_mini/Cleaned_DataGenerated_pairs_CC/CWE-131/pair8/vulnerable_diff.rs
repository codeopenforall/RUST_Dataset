use std::mem;
            let wrong_size = mem::size_of_val(&input);
            let layout = Layout::from_size_align(wrong_size, 1).unwrap();
            ptr::copy_nonoverlapping(input.as_ptr(), buf, wrong_size);
            String::from_raw_parts(buf, wrong_size, wrong_size)
