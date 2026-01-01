        let expected = {
            let mut tmp = Vec::with_capacity(data.len());
            tmp.extend_from_slice(&data);
            tmp
        };
                let _ = inst_clone.copy_data(&dclone);
