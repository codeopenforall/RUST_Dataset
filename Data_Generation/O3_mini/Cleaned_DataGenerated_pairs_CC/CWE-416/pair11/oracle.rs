#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_memory_access() {
        let resource = allocate();
        let value = unsafe { 
            #[allow(unused_unsafe)]
            resource.obtain() 
        };
        assert_eq!(value, 42, "The obtained value must be 42");
    }
}
