use peerage_hash::hasher::PeerageHash;
use rand::{thread_rng, Rng};



fn random_array() -> [u8; 1024] {
    let mut arr = [0u8; 1024];

    let mut rng = thread_rng();

    for i in 0..1024 {
        let x: u8 = rng.gen();

        if x % 2 == 0 {
            arr[i] = 0;
        } else {
            arr[i] = 1;
        }
        
        
    }

    arr
}



fn main() {
    

    let v = random_array();

    let mut hasher = PeerageHash::new(v);

    hasher.operate_rounds();

    let mut hasher3 = PeerageHash::new(v);
    hasher3.operate_rounds();


    println!("{:?} {}", hasher.get_hex_hash_output(), hasher3.get_hex_hash_output());

}
