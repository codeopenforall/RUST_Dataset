#[cfg(test)]
mod tests {
    // Bring the Purchase structure into scope.
    use super::Purchase;

    #[test]
    fn test_total_calculation() {
        // For an order with price 99, discount 20, and tax 8:
        // The correct calculation should be:
        // discount = 99 * 20 / 100 = 19,
        // tax = 99 * 8 / 100 = 7,
        // total = 99 - 19 + 7 = 87.
        let order = Purchase { price: 99, discount: 20, tax: 8 };
        let total = order.calculate();
        assert_eq!(total, 87, "Computed total {} does not match the expected value 87", total);
    }
}