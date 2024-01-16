use cid::Cid;
use multihash::Multihash;

use std::fs;

pub fn _file_to_cid(file_path: &str) -> cid::Result<Cid> {
    let contents = fs::read(file_path).expect("Should have been able to read the file");
    println!("contents as_slice: {:?}", contents.as_slice());

    let content = bytes_to_cid(contents.as_slice()).unwrap();
    Ok(content)
}

pub fn bytes_to_cid(bytes: &[u8]) -> cid::Result<Cid> {
    bytes_to_cid_v1(bytes)
}

fn bytes_to_cid_v1(bytes: &[u8]) -> cid::Result<Cid> {
    const SHA2_256: u64 = 0x12;
    const RAW: u64 = 0x55;
    let hash = Multihash::<64>::wrap(SHA2_256, bytes);
    let mh = hash.unwrap();
    let cid = Cid::new_v1(RAW, mh);
    Ok(cid)
}

fn _cid_to_vec(cid: Cid) -> Vec<u8> {
    let bytes = &cid.to_bytes()[4..];
    bytes.to_vec()
}

fn _cid_to_str(cid: Cid) -> String {
    let vec = _cid_to_vec(cid);
    let result = String::from_utf8(vec.to_vec()).unwrap();
    result.to_owned()
}
