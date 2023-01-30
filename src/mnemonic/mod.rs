use bip39::{Language, Mnemonic, MnemonicType};

pub fn generate(word_count: Option<MnemonicType>, language: Option<Language>) -> Mnemonic {
    let mnemonic_type = word_count.unwrap_or(MnemonicType::Words12);
    let lang = language.unwrap_or(Language::English);

    Mnemonic::new(mnemonic_type, lang)
}

#[cfg(test)]
mod tests {
    use crate::mnemonic::generate;
    use bip39::MnemonicType;

    #[test]
    fn should_default_to_12_english_words() {
        let mnemonic = generate(None, None);
        assert_eq!(mnemonic.phrase().split(" ").count(), 12);
    }

    #[test]
    fn should_match_needed_word_count() {
        let mnemonic = generate(Some(MnemonicType::Words21), None);
        assert_eq!(mnemonic.phrase().split(" ").count(), 21);
    }
}
