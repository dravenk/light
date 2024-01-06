use bip32::{secp256k1::ecdsa::Signature, Mnemonic, Prefix, XPrv};
use rand_core::OsRng;
// Sign and verify an example message using the derived keys.
use bip32::secp256k1::ecdsa::signature::{Signer, Verifier};

// use crate::wallet_phrase::get_mnemonic_by_strength;

// #[derive(Debug, Default, Clone, Serialize, Deserialize)]
// pub struct TwentyFourWords {
//     pub words: [String; 24],
// }

// pub fn get_24_words() -> TwentyFourWords {
//     // let mnemonic = Mnemonic::random(&mut OsRng, Default::default());
//     // let words = mnemonic.phrase().to_string();
//     let mnemonic = get_mnemonic_by_strength("24");
//     let words = mnemonic.split(" ").collect::<Vec<&str>>();
//     let mut words_array: [String; 24] = Default::default();
//     for (i, word) in words.iter().enumerate() {
//         words_array[i] = word.to_string();
//     }
//     let words = TwentyFourWords { words: words_array };
//     println!("words: {:?}", words.clone());
//     words
//     // TwentyFourWords::default()
// }
pub fn create_priv_k() -> Result<Signature, bip32::Error> {
    // Generate random Mnemonic using the default language (English)
    let mnemonic = Mnemonic::random(&mut OsRng, Default::default());

    // Derive a BIP39 seed value using the given password
    let seed = mnemonic.to_seed("password");

    // Derive the root `XPrv` from the `seed` value
    let root_xprv = XPrv::new(&seed)?;
    assert_eq!(root_xprv, XPrv::derive_from_path(&seed, &"m".parse()?)?);

    // Derive a child `XPrv` using the provided BIP32 derivation path
    let child_path = "m/0/2147483647'/1/2147483646'";
    let child_xprv = XPrv::derive_from_path(&seed, &child_path.parse()?)?;

    // Get the `XPub` associated with `child_xprv`.
    let child_xpub = child_xprv.public_key();

    // Serialize `child_xprv` as a string with the `xprv` prefix.
    let child_xprv_str = child_xprv.to_string(Prefix::XPRV);
    assert!(child_xprv_str.starts_with("xprv"));

    // Serialize `child_xpub` as a string with the `xpub` prefix.
    let child_xpub_str = child_xpub.to_string(Prefix::XPUB);
    assert!(child_xpub_str.starts_with("xpub"));

    // Get the ECDSA/secp256k1 signing and verification keys for the xprv and xpub
    let signing_key = child_xprv.private_key();
    let verification_key = child_xpub.public_key();

    let example_msg = b"Hello, world!";
    let signature: Signature = signing_key.sign(example_msg);
    assert!(verification_key.verify(example_msg, &signature).is_ok());
    Ok(signature)
}
