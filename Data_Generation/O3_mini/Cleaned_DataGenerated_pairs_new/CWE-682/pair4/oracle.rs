#[cfg(test)]
mod tests {
    use super::Purchase;
    #[test]
    fn test_total_calculation() {
        let order = Purchase { price: 99, discount: 20, tax: 8 };
        let total = order.calculate();
        assert_eq!(total, 87, "Computed total {} does not match the expected value 87", total);
    }
}
