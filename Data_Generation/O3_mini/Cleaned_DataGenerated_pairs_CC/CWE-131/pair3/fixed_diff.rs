        let buffer_size = input.len() + 1;
            *buffer.as_mut_ptr().add(input.len()) = 0;
        buf[..data.len()].iter().fold(0u8, |acc, &x| acc.wrapping_add(x))
