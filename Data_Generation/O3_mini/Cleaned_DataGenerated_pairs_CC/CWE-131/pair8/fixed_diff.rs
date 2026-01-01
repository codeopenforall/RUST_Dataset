            let correct_size = input.len();
            let layout = Layout::from_size_align(correct_size, 1).unwrap();
            ptr::copy_nonoverlapping(input.as_ptr(), buf, correct_size);
            String::from_raw_parts(buf, correct_size, correct_size)
