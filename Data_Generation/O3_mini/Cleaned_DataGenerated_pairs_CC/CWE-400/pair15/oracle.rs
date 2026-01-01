#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn oracle() {
        let excessive = 2000;
        let srv = Server::new();
        let res = srv.run(excessive);
        assert!(res.is_err(), "Expected rejection for task count above limit");
    }
}
