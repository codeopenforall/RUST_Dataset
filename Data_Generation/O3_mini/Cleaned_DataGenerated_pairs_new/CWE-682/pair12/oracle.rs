#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn discount_test() {
        let order = Order { price: 200, discount: 15 };
        let result = unsafe { order.total() };
        assert_eq!(result, 170, "Discount calculation is incorrect");
    }
}
