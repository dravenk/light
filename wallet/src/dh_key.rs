///
/// [dependencies]
/// bip32 = {version="0.5.1", features = ["alloc"]}
/// rand_core = {version="0.6.4",features = ['getrandom']}
/// x25519-dalek = {version = "2.0.0", features = ["static_secrets"]}
/// 
use bip32::{secp256k1::ecdsa::Signature, Mnemonic,PrivateKey, Prefix,  Seed, XPrv};
use rand_core::OsRng;
use x25519_dalek::{x25519,PublicKey, StaticSecret};

fn seed_to_dh() {
    // 分别两个不同私钥种子 seed1 和 seed2，代表由不同的两个人持有
    let seed1 = get_seed();
    let seed2 = get_seed();
    
    // 断言 seed1 不同于 seed2
    assert_ne!(seed1.as_bytes(), seed2.as_bytes());
    
    // 持有人 seed1 根据持有的私钥 seed1 派生任意私钥，并使用私钥创建创建对应的公钥 pub1
    let child_path = "m/44'/0'/0'/0/0'";
    let (sec1, pub1) = seed_to_key(seed1,child_path);
    println!("sec1: {:?}", sec1.to_bytes());
    println!("pub1: {:?}", pub1.to_bytes());

    // 持有人 seed2 根据持有的私钥 seed2 派生任意私钥，并使用私钥创建创建对应的公钥 pub2
    let any_path = "m/11'/0'/0'/0/0'";
    let (sec2, pub2) =  seed_to_key(seed2,any_path);
    println!("sec2: {:?}", sec2.to_bytes());
    println!("pub2: {:?}", pub2.to_bytes());

    
    // 断言 私钥1 不同于 私钥2
    assert_ne!(sec1.to_bytes(), sec2.to_bytes());
    // 断言 公钥1 不同于 公钥2
    assert_ne!(pub1.to_bytes(), pub2.to_bytes());

    // 经过DH协议交换后，seed1得到的共享密钥
    let shard1 = x25519(sec1.to_bytes(), pub2.to_bytes());
    // 经过DH协议交换后，seed2得到的共享密钥
    let shard2 = x25519(sec2.to_bytes(), pub1.to_bytes());
    
    println!("shard1: {:?}", shard1);
    println!("shard2: {:?}", shard2);

    // 断言 seed1 持有人 和 seed2 持有人经过 DH 协议，得到的共享密钥
    assert_eq!(shard1, shard2);
}

pub fn get_seed(password:Option(&str)) -> Seed {
    let phrase = get_mnemonic();
    // Derive a BIP39 seed value using the given password
    let seed = phrase.to_seed(password.unwrap_or_default());
    seed
}

pub fn seed_to_key(seed:Seed, child_path:&str) -> (StaticSecret, PublicKey) {
    let child_xprv = XPrv::derive_from_path(&seed, &child_path.parse().unwrap()).unwrap();
    let xprv = child_xprv.to_bytes();
    let child_priv_key: [u8; 32] = xprv[0..32].try_into().unwrap();

    let secret_key = StaticSecret::from(child_priv_key);
    let public_key = PublicKey::from(&secret_key);
    (secret_key, public_key)
}

pub fn get_mnemonic() -> Mnemonic {
    // Generate random Mnemonic using the default language (English)
    let language = Default::default();
    let mnemonic = Mnemonic::random(&mut OsRng, language);
    let phrase = Mnemonic::new(mnemonic.phrase(), language);
    phrase.unwrap()
}

pub fn sedd_to_dh_key(seed: [u8; 32]) -> (StaticSecret, PublicKey) {
    let secret_key = StaticSecret::from(seed);
    let public_key = PublicKey::from(&secret_key);
    (secret_key, public_key)
}

