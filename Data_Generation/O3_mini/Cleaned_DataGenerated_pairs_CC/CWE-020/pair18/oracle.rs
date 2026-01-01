#[cfg(test)]
mod tests {
    use super::Holder;
    #[test]
    fn test_extraction() {
        let data = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let holder = Holder::new(data).expect("Initialization failed");
        let result = holder.process(20, 10);
        assert_eq!(result, "UVWXYZ", "The extracted substring did not match the expected safe output.");
    }
}
