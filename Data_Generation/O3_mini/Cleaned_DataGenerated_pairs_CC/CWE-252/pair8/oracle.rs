#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io;
    #[test]
    fn test_oracle() {
        let _ = fs::remove_file("log.txt");
        {
            let processor = Processor;
            #[allow(unused_must_use)]
            {
                processor.run("normal input");
            }
            let content = fs::read_to_string("log.txt").unwrap_or_default();
            assert_eq!(content, "Processed: normal input", "The log file should contain the correct message.");
        }
        {
            let processor = Processor;
            let res = processor.run("fail");
            match res {
                Ok(_) => {
                    let content = fs::read_to_string("log.txt").unwrap_or_default();
                    assert_ne!(content, "Processed: fail", "Vulnerability: error was ignored; incorrect file state.");
                },
                Err(e) => {
                    assert!(e.to_string().contains("simulated failure"), "Fixed version should report the simulated failure.");
                },
            }
        }
    }
}
