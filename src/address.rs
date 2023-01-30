use bitcoin_cash::{Prefix, Pubkey};

#[derive(Debug)]
pub struct Address {
    pub cash_address: String,
    pub slp_address: String,
}

impl Address {
    pub fn from_bitcoin_public_key(pubkey: &bitcoin::PublicKey) -> Address {
        let cash_pubkey = Pubkey::from_slice(&pubkey.to_bytes());

        let cash_address = bitcoin_cash::Address::from_pk(Prefix::BitcoinCash, &cash_pubkey)
            .cash_addr()
            .to_owned();
        let slp_address = bitcoin_cash::Address::from_pk(Prefix::SimpleLedger, &cash_pubkey)
            .cash_addr()
            .to_owned();

        Address {
            cash_address,
            slp_address,
        }
    }

    pub fn from_bitcoin_cash_public_key(cash_pubkey: &Pubkey) -> Address {
        let cash_address = bitcoin_cash::Address::from_pk(Prefix::BitcoinCash, &cash_pubkey)
            .cash_addr()
            .to_owned();
        let slp_address = bitcoin_cash::Address::from_pk(Prefix::SimpleLedger, &cash_pubkey)
            .cash_addr()
            .to_owned();

        Address {
            cash_address,
            slp_address,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::accounts;

    use super::*;

    #[test]
    fn should_success_using_bitcoin_crate() {
        let account = accounts::Account::new().unwrap();
        let pubkey = account.to_public_key();

        let address = Address::from_bitcoin_public_key(&pubkey);

        assert!(address.cash_address.starts_with("bitcoincash:"));
        assert!(address.slp_address.starts_with("simpleledger:"));
    }

    #[test]
    fn should_success_using_bitcoin_cash_crate() {
        let account = accounts::Account::new().unwrap();
        let pubkey = account.to_public_key();

        let cash_pubkey = bitcoin_cash::Pubkey::from_slice(&pubkey.to_bytes());
        let address = Address::from_bitcoin_cash_public_key(&cash_pubkey);

        assert!(address.cash_address.starts_with("bitcoincash:"));
        assert!(address.slp_address.starts_with("simpleledger:"));
    }
}
