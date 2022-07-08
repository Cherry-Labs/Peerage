use peerage_hash::hasher::PeerageHash;


fn main() {
    let mut v: Vec<u8> = (0u8..=255u8).collect();

    v.push(0u8);

    let mut hasher = PeerageHash::new(v);

    hasher.operate_rounds();

    println!("{:?}", hasher.get_hex_hash());

}
