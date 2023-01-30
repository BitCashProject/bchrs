mod address;
mod keys;

use bitcoin::util::key::Error;
use bitcoin::{Network, PrivateKey};
use rand::RngCore;

use crate::address::Address;

#[derive(Debug)]
pub struct Account {
    private_key: PrivateKey,
    address: Address,
}

impl Account {
    pub fn new() -> Result<Account, Error> {
        let secp = bitcoin::secp256k1::Secp256k1::new();
        let mut osrng = rand::rngs::OsRng::default();
        let mut private_key = [0u8; 32];
        osrng.fill_bytes(&mut private_key);

        let private_key = PrivateKey::from_slice(&private_key, Network::Bitcoin)?;
        let address = Address::from_bitcoin_public_key(&private_key.public_key(&secp));

        Ok(Account {
            private_key,
            address,
        })
    }

    pub fn from_wif(wif: &str) -> Result<Account, Error> {
        let secp = bitcoin::secp256k1::Secp256k1::new();
        let private_key = PrivateKey::from_wif(wif)?;
        let address = Address::from_bitcoin_public_key(&private_key.public_key(&secp));

        Ok(Account {
            private_key,
            address,
        })
    }

    pub fn from_private_key(private_key: PrivateKey) -> Account {
        let secp = bitcoin::secp256k1::Secp256k1::new();
        let address = Address::from_bitcoin_public_key(&private_key.public_key(&secp));

        Account {
            private_key,
            address,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn account_generated_should_be_unique() {
        let account1 = Account::new().unwrap();
        let account2 = Account::new().unwrap();

        assert_ne!(account1.to_wif(), account2.to_wif());
    }

    #[test]
    fn imported_wif_must_have_correct_address() {
        let wif = "KxRwBk8Pn8BqCShuhHEVtBfBPnBe8uUPqBzGtnDNg46fB65s7h1f";
        let account = Account::from_wif(&wif).unwrap();

        assert_eq!(
            account.cash_addr(),
            "bitcoincash:qrx2mz55udyk2xqzelkcea0us35zk5jyss62y9lu84"
        );
        assert_eq!(
            account.slp_addr(),
            "simpleledger:qrx2mz55udyk2xqzelkcea0us35zk5jyssk3072uet"
        );
    }
}
