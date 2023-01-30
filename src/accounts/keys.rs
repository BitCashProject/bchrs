use super::Account;

impl Account {
    pub fn to_wif(&self) -> String {
        self.private_key.to_wif()
    }

    pub fn to_public_key(&self) -> bitcoin::PublicKey {
        let secp = bitcoin::secp256k1::Secp256k1::new();
        self.private_key.public_key(&secp)
    }
}
