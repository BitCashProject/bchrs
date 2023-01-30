use bip39::{Mnemonic, Seed};
use bitcoin::util::bip32::{ChildNumber, DerivationPath, ExtendedPrivKey};
use bitcoin::Network;

use crate::accounts::Account;
use crate::errors::ErrorKind;

#[derive(Debug)]
pub struct Wallet {
    extended_priv_key: ExtendedPrivKey,
}

impl Wallet {
    pub fn from_phrase(
        phrase: &str,
        password: Option<&str>,
        derivation_path: Option<DerivationPath>,
    ) -> Result<Wallet, ErrorKind> {
        use std::str::FromStr;

        let password = password.unwrap_or("");
        let mnemonic = Mnemonic::from_phrase(phrase, bip39::Language::English)
            .map_err(ErrorKind::map_bip39)?;

        let default_derivation_path = DerivationPath::from_str("m/44'/145'/0'/0").unwrap();
        let derivation_path = derivation_path.unwrap_or(default_derivation_path);

        let secp = bitcoin::secp256k1::Secp256k1::new();
        let seed = Seed::new(&mnemonic, password).as_bytes().to_owned();
        let extended_priv_key = ExtendedPrivKey::new_master(Network::Bitcoin, &seed)
            .map_err(ErrorKind::map_bip32)?
            .derive_priv(&secp, &derivation_path)
            .map_err(ErrorKind::map_bip32)?;

        Ok(Wallet { extended_priv_key })
    }

    pub fn account_at(&self, index: u32) -> Result<Account, ErrorKind> {
        let secp = bitcoin::secp256k1::Secp256k1::new();
        let derivation_path = ChildNumber::from_normal_idx(index).map_err(ErrorKind::map_bip32)?;

        let private_key = self
            .extended_priv_key
            .ckd_priv(&secp, derivation_path)
            .map_err(ErrorKind::map_bip32)?
            .to_priv();

        Ok(Account::from_private_key(private_key))
    }
}

#[cfg(test)]
mod tests {

    use crate::errors::ErrorKind;

    use super::Wallet;

    #[test]
    fn should_be_able_to_import_from_phrase() {
        let mnemonic =
            "tooth worth shuffle maze traffic advice burden absurd glass ancient usage field";
        let wallet = Wallet::from_phrase(&mnemonic, None, None);

        assert!(wallet.is_ok());
    }

    #[test]
    fn should_error_if_not_valid_phrase() {
        let mnemonic = "tooth worth invalid_word shuffle maze traffic";
        let wallet = Wallet::from_phrase(&mnemonic, None, None);

        assert!(wallet.is_err());

        let error = wallet.unwrap_err();
        match error {
            ErrorKind::Bip39InvalidWord => {}
            _ => panic!("{:#?}", error),
        };
    }

    #[test]
    fn should_have_correct_derived_account() {
        let phrase = "grass chest father crew latin box fee shove brother blossom volume scissors"
            .to_string();
        let wallet = Wallet::from_phrase(&phrase, None, None).unwrap();

        let account = wallet.account_at(0).unwrap();
        assert_eq!(
            account.cash_addr(),
            "bitcoincash:qqzw2uwyu0ytazgajcv3g0ymhg3arwdrfvsw5tnh9v"
        );

        let account = wallet.account_at(1).unwrap();
        assert_eq!(
            account.cash_addr(),
            "bitcoincash:qpz992ts0lk5x4x64mcdyfkfck70xcyg2gz3gxm3jf"
        );
    }
}
