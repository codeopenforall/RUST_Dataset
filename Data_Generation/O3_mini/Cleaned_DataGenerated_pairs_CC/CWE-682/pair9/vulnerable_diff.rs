        let ptr: *const u32 = &sum;
        let unsafe_sum = unsafe { *ptr };
        let result = unsafe_sum / count * self.scale;
