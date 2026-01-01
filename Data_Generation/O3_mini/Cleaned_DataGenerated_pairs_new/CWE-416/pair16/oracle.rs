#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dangling_access() {
        let mut resource = Holder::new(12345);
        unsafe {
            resource.deallocate();
        }
        let result = process(&resource);
        assert_eq!(result, -1, "The resource was accessed after being freed!");
    }
}
