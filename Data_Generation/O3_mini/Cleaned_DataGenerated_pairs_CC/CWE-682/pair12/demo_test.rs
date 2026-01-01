//////////////////////////////////////////
// Test Oracle for the Discount Calculation
//
// This test oracle function is designed to be used in a unit test setting.
// It creates an Order with a price of 200 and a discount of 15. The expected
// result is (200 * (100 - 15)) / 100 = 170. When run against the vulnerable version,
// the test will fail, while it will pass when run against the fixed version.
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn discount_test() {
        let order = Order { price: 200, discount: 15 };
        let result = unsafe { order.total() };
        // Expected total: (200 * 85) / 100 = 170
        assert_eq!(result, 170, "Discount calculation is incorrect");
    }
}