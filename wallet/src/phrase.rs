pub extern crate serde;

use bip32::secp256k1::ecdsa::SigningKey;
use bip32::{Seed, XPrv};
use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

use serde::Serialize;
use util::{self, appdata};

#[derive(Debug, Default, Clone, Serialize)]
pub struct PrivKey {
    pub strength: String,
    pub phrase: String,
    pub key_path: String,
}

impl PrivKey {
    pub fn set_strength(&mut self, strength: String) {
        self.strength = strength;
    }
    pub fn set_phrase(&mut self, phrase: String) {
        self.phrase = phrase;
    }
    pub fn set_key_path(&mut self, key_path: String) {
        self.key_path = key_path;
    }
}

pub fn get_mnemonic_by_strength(_strength: &str) -> PrivKey {
    let strength = "24";
    let mut priv_key = PrivKey::default();
    let key_path = appdata::key_path();
    priv_key.set_key_path(key_path);

    let ph = get_phrase_from_file(priv_key.key_path.clone());
    if !ph.is_empty() {
        priv_key.phrase = ph.to_owned();
        return priv_key;
    }

    let _word_count: usize = FromStr::from_str(strength).unwrap();
    let (phrase, _) = super::seed::get_phrase_seed();

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

pub fn get_root_xprv(seed: &Seed) -> SigningKey {
    let root_xprv = XPrv::new(&seed).unwrap();
    let private_key = root_xprv.private_key();
    private_key.to_owned()
}
