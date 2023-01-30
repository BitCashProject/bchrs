pub mod accounts;
pub mod address;
pub mod errors;
pub mod mnemonic;
pub mod wallet;

// let bitcoin_secp = bitcoin::secp256k1::Secp256k1::new();
// let pk = bitcoin::PrivateKey::from_wif("L3dNdXnX3h16zBRFNGcFpQpaeM22oev5sJ3EsyBMubyxNuTAQAtc")
//     .unwrap();

// let pubkey = pk.public_key(&bitcoin_secp);
// let address = bitcoin::Address::p2pkh(&pubkey, bitcoin::Network::Bitcoin);

// println!("Privkey: {}", pk.to_wif());
// println!("Pubkey: {}", pubkey.to_string());
// println!("address: {}", address.to_string());

// let cash_pubkey = bitcoin_cash::Pubkey::from_slice(&pubkey.to_bytes());
// println!(
//     "address: {}",
//     bitcoin_cash::Address::from_pk(Prefix::SimpleLedger, &cash_pubkey).cash_addr()
// );
