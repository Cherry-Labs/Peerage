use peerage_utils::bin_utils::QuadrupleWord;
use peerage_rand::quadrupleword::RandomQuadrupleWord;
use peerage_hash::hasher::PeerageHash;
use crate::prime_number_db::get_prime_num;
use crate::math::{lcm, are_coprime, mod_inverse};


pub type ResSign = std::result::Result<Vec<QuadrupleWord>, ()>;
pub type ResEncrypt = std::result::Result<Vec<QuadrupleWord>, ()>;
pub type ResDecrypt = std::result::Result<Vec<QuadrupleWord>, ()>;



#[derive(Clone, Copy)]
pub enum Key {
    Nil,
    PublicKey(QuadrupleWord, QuadrupleWord),
    PrivateKey(QuadrupleWord, QuadrupleWord)
}




impl  Default for Key {
    fn default() -> Self {
        Self::Nil
    }
}

impl Key {
    pub fn new_public(n: u128, e: u128) -> Self {
        let n = QuadrupleWord::from_u128(n);
        let e = QuadrupleWord::from_u128(e);

        Self::PublicKey(n, e)
    }

    pub fn new_private(n: u128, d: u128) -> Self {
        let n = QuadrupleWord::from_u128(n);
        let d = QuadrupleWord::from_u128(d);

        Self::PublicKey(n, d)
    }

    pub fn sign(&self, data: Vec<QuadrupleWord>) -> ResSign {
        match self {
            Key::Nil => Err(()),
            Key::PublicKey(n, e) => {
                let hashes = data.into_iter()
                            .map(|x|{
                                let hash = PeerageHash::new_from_quadrupleworld(x);
                                let hash_final = hash.get_final_output();
                                hash_final % *n
                            })
                            .collect::<Vec<QuadrupleWord>>();

            Ok(hashes)
            },
            Key::PrivateKey(_, _) => Err(()),
        }
    }

    pub fn verify_signature(&self, data: Vec<QuadrupleWord>, signatures: Vec<QuadrupleWord>) -> bool {
        match self {
            Key::Nil => false,
            Key::PublicKey(n, e) => {
               for (data, sig) in data.into_iter().zip(signatures.into_iter()) {
                    match sig != {
                        let hash = PeerageHash::new_from_quadrupleworld(data);
                        let hash_final = hash.get_final_output();
                        hash_final % *n
                    }
                    {
                        true => return false,
                        false => continue,
                    }
               }

               true
            },
            Key::PrivateKey(_, _) => false,
        }
    }

    pub fn encrypt(&self, data: Vec<QuadrupleWord>) -> ResEncrypt {
        match self {
            Key::Nil => Err(()),
            Key::PublicKey(n, e) => {
                let res = data.into_iter()
                    .map(|x| x.pow_self(*e) % *n)
                    .collect::<Vec<QuadrupleWord>>();


                Ok(res)
            },
            Key::PrivateKey(_, _) => Err(()),
        }
    }

    pub fn decrypt(&self, enc_data: Vec<QuadrupleWord>) -> ResDecrypt {
        match self {
            Key::Nil => Err(()),
            Key::PublicKey(_, _) => Err(()),
            Key::PrivateKey(n, d) => {
                let res = enc_data.into_iter()
                .map(|x| x.pow_self(*d) % *n)
                .collect::<Vec<QuadrupleWord>>();

                Ok(res)
            },
        }
    }
}

#[derive(Clone, Copy)]
pub struct PeerageKeyPair<const M: usize> {
    pubk: [Key; M],
    privk: [Key; M],
    p: [u128; M],
    q: [u128; M],
    n: [u128; M],
    e: [u128; M],
    d: [u128; M],

}

impl<const M: usize> PeerageKeyPair<M> {
    pub fn new_empty() -> Self {
        let pubk: [Key; M] = [Key::default(); M];
        let privk: [Key; M] = [Key::default(); M];
        let p: [u128; M] = [0u128; M];
        let q: [u128; M] = [0u128; M];
        let n: [u128; M] = [0u128; M];
        let e: [u128; M] = [0u128; M];
        let d: [u128; M] = [0u128; M];

        Self { 
            pubk, 
            privk,
            p, 
            q, 
            n, 
            e, 
            d,
         }
    }

    fn select_p_q() -> ([u128; M], [u128; M]) {
        let mut p = [0u128; M];
        let mut q = [0u128; M];

        for i in 0..M {
            p[i] = get_prime_num();
            q[i] = get_prime_num();
        }

        (p, q)
    }

    fn compute_pxq(p: [u128; M], q: [u128; M]) -> [u128; M] {
        let mut ret = [0u128; M];

        for i in 0..M {
            ret[i] = p[i] * q[i]
        }

        ret
    }

    fn compute_pq_lcm(p: [u128; M], q: [u128; M]) -> [u128; M] {
        let mut ret = [0u128; M];

        for i in 0..M {
            ret[i] = lcm(p[i] - 1, q[i] - 1)
        }

        ret

    }

    fn get_coprime_sigle(m: u128) -> u128 {
        for i in 0u128..m {
            match are_coprime(m, i) {
                true => return i,
                false => continue,
            }
        }

        0u128
    }

    fn get_coprimes(m: [u128; M]) -> [u128; M] {
        let mut ret = [0u128; M];

        for i in 0..M {
            ret[i] = Self::get_coprime_sigle(m[i])
        }

        ret
    }

    fn get_mod_inverse(a: [u128; M], m: [u128; M]) -> [u128; M] {
        let mut ret = [0u128; M];

        for i in 0..M {
            ret[i] = mod_inverse(a[i] - 1, m[i] - 1)
        }

        ret
    }

    pub fn make_public_keys(n: [u128; M], e: [u128; M]) -> [Key; M] {
        let mut ret = [Key::default(); M];

        for i in 0..M {
            ret[i] = Key::new_public(n[i], e[i])
        }

        ret
    }

    pub fn make_private_keys(n: [u128; M], d: [u128; M]) -> [Key; M] {
        let mut ret = [Key::default(); M];

        for i in 0..M {
            ret[i] = Key::new_private(n[i], d[i])
        }

        ret
    }
}



impl<const M: usize> Default for PeerageKeyPair<M> {
    fn default() -> Self {
        Self::new_empty()
    }
}