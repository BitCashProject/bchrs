use super::Account;

impl Account {
    pub fn cash_addr(&self) -> String {
        self.address.cash_address.to_owned()
    }

    pub fn slp_addr(&self) -> String {
        self.address.slp_address.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_correct_address() {
        let account1 = Account::new().unwrap();
        assert!(account1.cash_addr().starts_with("bitcoincash:"));
        assert!(account1.slp_addr().starts_with("simpleledger:"));
    }
}
