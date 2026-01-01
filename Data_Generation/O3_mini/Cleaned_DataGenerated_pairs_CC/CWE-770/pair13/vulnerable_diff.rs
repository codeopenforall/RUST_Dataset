            let ptr = Vec::<u8>::with_capacity(1).as_mut_ptr();
            *ptr.offset(0) = (task_id % 256) as u8;
