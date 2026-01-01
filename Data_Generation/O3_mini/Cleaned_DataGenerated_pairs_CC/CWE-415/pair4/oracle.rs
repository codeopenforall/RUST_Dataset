#[cfg(test)]
mod tests {
    use super::run;
    #[test]
    fn test_memory_release() {
        let result = std::panic::catch_unwind(|| {
            run();
        });
        assert!(result.is_ok(), "Resource deallocation failed due to double free");
    }
}
