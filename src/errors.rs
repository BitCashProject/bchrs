use bip39::ErrorKind as Bip39ErrorKind;
use bitcoin::util::bip32::Error as Bip32ErrorKind;

#[derive(Debug)]
pub enum ErrorKind {
    Bip39InvalidChecksum,
    Bip39InvalidWord,
    Bip39InvalidKeysize(usize),
    Bip39InvalidWordLength(usize),
    Bip39InvalidEntropyLength(usize, bip39::MnemonicType),

    Bip32CannotDeriveFromHardenedKey,
    Bip32Secp256k1(bitcoin::secp256k1::Error),
    Bip32InvalidChildNumber(u32),
    Bip32InvalidChildNumberFormat,
    Bip32InvalidDerivationPathFormat,
    Bip32UnknownVersion([u8; 4]),
    Bip32WrongExtendedKeyLength(usize),
    Bip32Base58(bitcoin::util::base58::Error),
    Bip32Hex(bitcoin::hashes::hex::Error),
}

impl ErrorKind {
    pub fn from_bip39(error: anyhow::Error) -> Self {
        let casting = error.downcast_ref::<bip39::ErrorKind>();

        if casting.is_some() {
            return match casting.unwrap() {
                Bip39ErrorKind::InvalidChecksum => ErrorKind::Bip39InvalidChecksum,
                Bip39ErrorKind::InvalidWord => ErrorKind::Bip39InvalidWord,
                Bip39ErrorKind::InvalidKeysize(a) => ErrorKind::Bip39InvalidKeysize(*a),
                Bip39ErrorKind::InvalidWordLength(a) => ErrorKind::Bip39InvalidWordLength(*a),
                Bip39ErrorKind::InvalidEntropyLength(a, b) => {
                    ErrorKind::Bip39InvalidEntropyLength(*a, *b)
                }
            };
        }

        panic!("unknown error variant");
    }

    pub fn from_bip32(error: Bip32ErrorKind) -> Self {
        match error {
            Bip32ErrorKind::CannotDeriveFromHardenedKey => {
                ErrorKind::Bip32CannotDeriveFromHardenedKey
            }
            Bip32ErrorKind::Secp256k1(e) => ErrorKind::Bip32Secp256k1(e),
            Bip32ErrorKind::InvalidChildNumber(a) => ErrorKind::Bip32InvalidChildNumber(a),
            Bip32ErrorKind::InvalidChildNumberFormat => ErrorKind::Bip32InvalidChildNumberFormat,
            Bip32ErrorKind::InvalidDerivationPathFormat => {
                ErrorKind::Bip32InvalidDerivationPathFormat
            }
            Bip32ErrorKind::UnknownVersion(a) => ErrorKind::Bip32UnknownVersion(a),
            Bip32ErrorKind::WrongExtendedKeyLength(usize) => {
                ErrorKind::Bip32WrongExtendedKeyLength(usize)
            }
            Bip32ErrorKind::Base58(e) => ErrorKind::Bip32Base58(e),
            Bip32ErrorKind::Hex(e) => ErrorKind::Bip32Hex(e),
            _ => panic!("unknown error"),
        }
    }
}

impl ErrorKind {
    pub fn map_bip39(e: anyhow::Error) -> ErrorKind {
        ErrorKind::from_bip39(e)
    }

    pub fn map_bip32(e: Bip32ErrorKind) -> ErrorKind {
        ErrorKind::from_bip32(e)
    }
}
