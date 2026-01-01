        std::ptr::copy_nonoverlapping(data.as_ptr(), self.ptr, data.len());
