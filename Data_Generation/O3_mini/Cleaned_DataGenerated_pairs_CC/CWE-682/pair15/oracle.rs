#[cfg(test)]
mod tests {
    use super::SensorData;
    #[test]
    fn test_calculation() {
        let sensor = SensorData { readings: vec![10, 20, 30, 40] };
        let result = sensor.process();
        assert_eq!(result, 150, "Calculation result should be 150 based on proper arithmetic.");
    }
}
