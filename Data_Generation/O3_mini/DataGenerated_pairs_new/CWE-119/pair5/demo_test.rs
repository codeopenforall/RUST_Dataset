/*
This oracle test is designed to verify the integrity of the memory block.
It calls the run() function and asserts that the sentinel remains intact.
For the vulnerable version run() will return false (sentinel corrupted)
and this test will fail.
For the fixed version, run() returns true and the test passes.
*/
#[cfg(test)]
mod tests {
    use super::run;
    #[test]
    fn oracle_test() {
        let integrity = run();
        assert!(integrity, "Integrity check failed: sentinel was corrupted");
    }
}