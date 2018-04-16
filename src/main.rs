extern crate multihash;
extern crate cid;

fn main() {
    // Raw hash to encode:
    // 0xd4e56740f876aef8c010b86a40d5f56745a118d0906a34e69aec8c0db1cb8fa3
    
    // The multihash crate doesn't let you actually get the multihash as bytes
    // starting from an existing hash, just from data, which isn't what we want.
    // Bah.
    // So we do it ourself: it's just two bytes, one specifying the hash algorithm's
    // ID and one specifying the size of the hash.
    let code = multihash::Hash::Keccak256.code();
    let size = multihash::Hash::Keccak256.size();
    let bytes_to_convert: Vec<u8> = vec![
        code, size,
        0xd4, 0xe5, 0x67, 0x40,
        0xf8, 0x76, 0xae, 0xf8,
        0xc0, 0x10, 0xb8, 0x6a,
        0x40, 0xd5, 0xf5, 0x67,
        0x45, 0xa1, 0x18, 0xd0,
        0x90, 0x6a, 0x34, 0xe6,
        0x9a, 0xec, 0x8c, 0x0d,
        0xb1, 0xcb, 0x8f, 0xa3
    ];

    // Now we just turn our multihash
    let c2 = cid::Cid::new(cid::Codec::Raw, cid::Version::V1, bytes_to_convert.as_ref());
    println!("C2: {}", c2.to_string());
}
