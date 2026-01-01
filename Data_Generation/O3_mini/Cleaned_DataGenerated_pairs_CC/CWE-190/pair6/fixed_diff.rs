use std::convert::TryFrom;
        let safe_input = i32::try_from(input).unwrap_or(i32::MAX);
        let addition = safe_input.saturating_mul(self.factor);
        base.saturating_add(addition)
