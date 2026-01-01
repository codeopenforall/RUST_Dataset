        let actual = if req > allowed { allowed } else { req };
                std::ptr::copy_nonoverlapping(source.as_ptr(), buf.as_mut_ptr(), actual);
