use bip32::{Mnemonic, Seed};
use rand_core::OsRng;

pub fn get_mnemonic() -> Mnemonic {
    // Generate random Mnemonic using the default language (English)
    let language = Default::default();
    let mnemonic = Mnemonic::random(&mut OsRng, language);
    let phrase = Mnemonic::new(mnemonic.phrase(), language);
    phrase.unwrap()
}

pub fn get_phrase_seed() -> (Mnemonic, Seed) {
    let phrase = get_mnemonic();
    // Derive a BIP39 seed value using the given password
    let seed = phrase.to_seed("");
    (phrase.to_owned(), seed)
}

pub fn get_seed(password: Option<&str>) -> Seed {
    let phrase = get_mnemonic();
    // Derive a BIP39 seed value using the given password
    let seed = phrase.to_seed(password.unwrap_or_default());
    seed
}
