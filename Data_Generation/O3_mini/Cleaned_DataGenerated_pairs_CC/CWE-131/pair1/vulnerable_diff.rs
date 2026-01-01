        let msg_struct_size = mem::size_of_val(&record.message);
        let total_size = header_size + msg_struct_size;
            msg_struct_size,
