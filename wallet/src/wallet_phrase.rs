pub extern crate core;
pub extern crate serde;

use bip32::secp256k1::ecdsa::signature::{Signer, Verifier};
use bip32::secp256k1::ecdsa::SigningKey;
use bip32::{secp256k1::ecdsa::Signature, Mnemonic, Prefix, XPrv};
use bip32::{PrivateKey, Seed};
use rand_core::OsRng;
use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

use serde::Serialize;
use util::{self, appdata};

#[derive(Debug, PartialEq, Default, Clone, Serialize)]
pub struct TwentyFourWords {
    pub words: [String; 24],
}

pub fn get_24_words() -> TwentyFourWords {
    println!("get_24_words");
    // let mnemonic = Mnemonic::random(&mut OsRng, Default::default());
    // let words = mnemonic.phrase().to_string();
    let words = self::get_mnemonic_by_strength("24");
    let words = words.phrase.split(" ").collect::<Vec<&str>>();
    let mut words_array: [String; 24] = Default::default();
    for (i, word) in words.iter().enumerate() {
        words_array[i] = word.to_string();
    }
    let words = TwentyFourWords { words: words_array };
    println!("words: {:?}", words.clone());
    words
    // TwentyFourWords::default()
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct PrivKey {
    pub strength: String,
    pub phrase: String,
    pub key_path: String,
}

pub fn get_mnemonic_by_strength(_strength: &str) -> PrivKey {
    let strength = "24";
    let mut priv_key = PrivKey::default();
    let key_path = appdata::key_path();
    println!("key_path: {:?}", key_path);
    priv_key.key_path = key_path;

    let ph = get_phrase_from_file(priv_key.key_path.clone());
    if !ph.is_empty() {
        priv_key.phrase = ph.to_owned();
        return priv_key;
    }

    let _word_count: usize = FromStr::from_str(strength).unwrap();
    let (phrase, seed) = get_phrase_seed();
    // println!("seed: {:#?}", seed.as_bytes().to_ascii_lowercase());
    // phrase_to_file(&phrase);

    let xprv_key = get_root_xprv(&seed);
    let verification_key = xprv_key.public_key();
    let example_msg = b"Hello, world!";
    let signature: Signature = xprv_key.sign(example_msg);
    println!("signature: {:?}", signature.to_string());
    println!("verify: {:?}", verification_key.verify(example_msg, &signature).is_ok());

    // phrase.phrase().to_owned().to_string();
    priv_key.phrase = phrase.phrase().to_owned().to_string();
    priv_key
}

fn get_phrase_from_file(path: String) -> String {
    let file = fs::read(path);
    match file {
        Ok(file) => {
            let s: String = String::from_utf8_lossy(&file).to_string();
            let config: HashMap<String, String> = toml::from_str(&s).unwrap();
            println!("{:?}", config);
            config["phrase"].to_string()
        }
        Err(_) => "".to_string(),
    }
}

// fn phrase_to_file(phrase: &Mnemonic) {
//     let priv_key = PrivKey { phrase: phrase.phrase().to_owned().to_string() };
//     let toml_string = toml::to_string(&priv_key).expect("Could not encode TOML value");
//     // create directory
//     let db_path = db_dir();
//     fs::create_dir_all(db_path).expect("create dir error");
//     // write file
//     fs::write(KEYFILE_PATH, toml_string).expect("Could not write to file!");
// }

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

pub fn get_root_xprv(seed: &Seed) -> SigningKey {
    let root_xprv = XPrv::new(&seed).unwrap();
    let private_key = root_xprv.private_key();
    private_key.to_owned()
}

pub fn _priv_k() -> Result<Signature, bip32::Error> {
    let (_, seed) = get_phrase_seed();
    println!("root_xprv: {:?}", get_root_xprv(&seed));
    // let child_path = "m/0/2147483647'/1/2147483646'";
    let child_path = "m/44'/0'/0'/0/0'";
    // bip141 m/44'/0'/0'/0
    let child_xprv = XPrv::derive_from_path(&seed, &child_path.parse()?)?;
    println!("child_xprv: {:?}", child_xprv.to_string(Prefix::XPRV));
    // Get the `XPub` associated with `child_xprv`.
    let child_xpub = child_xprv.public_key();
    // println!("child_xpub: {:?}", child_xpub.to_string(Prefix::XPUB).to_string());
    // Serialize `child_xprv` as a string with the `xprv` prefix.
    let child_xprv_str = child_xprv.to_string(Prefix::XPRV);
    // println!("child_xprv_str: {:?}", child_xprv_str);
    assert!(child_xprv_str.starts_with("xprv"));

    // Serialize `child_xpub` as a string with the `xpub` prefix.
    let child_xpub_str = child_xpub.to_string(Prefix::XPUB);
    println!("child_xpub_str: {:?}", child_xpub_str);
    // assert!(child_xpub_str.starts_with("xpub"));

    // Get the ECDSA/secp256k1 signing and verification keys for the xprv and xpub
    let signing_key = child_xprv.private_key();
    let verification_key = child_xpub.public_key();

    let example_msg = b"Hello, world!";
    let signature: Signature = signing_key.sign(example_msg);
    println!("signature: {:?}", signature.to_string());
    assert!(verification_key.verify(example_msg, &signature).is_ok());
    Ok(signature)
}
