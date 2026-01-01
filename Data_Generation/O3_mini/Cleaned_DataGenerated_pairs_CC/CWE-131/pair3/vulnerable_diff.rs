use std::mem;
        let buffer_size = mem::size_of::<&str>(); 
        buf.iter().fold(0u8, |acc, &x| acc.wrapping_add(x))
