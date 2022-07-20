use peerage_utils::bin_utils::QuadrupleWord;
use peerage_rand::mersenne_twister::MerseneTwisterRand;
use crate::prime_number_db::get_prime_num;
use crate::math::{lcm, are_coprime, mod_inverse};

#[derive(Clone, Copy, Default)]
pub struct Key {
    n: QuadrupleWord,
    e: QuadrupleWord,
}

impl Key {
    pub fn new(n: u128, e: u128) -> Self {
        let n = QuadrupleWord::from_u128(n);
        let e = QuadrupleWord::from_u128(e);

        Self { n, e }
    }
}


#[derive(Clone, Copy)]
pub struct PeerageKeyPair<const M: usize> {
    pubk: [Key; M],
    privk: [Key; M],
    p: [u128; M],
    q: [u128; M],
    n: [u128; M],
    ctf: [u128; M],
    mmi: [u128; M],

}

impl<const M: usize> PeerageKeyPair<M> {


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
            ret[i] = Key::new(n[i], e[i])
        }

        ret
    }

    pub fn make_private_keys(n: [u128; M], d: [u128; M]) -> [Key; M] {
        let mut ret = [Key::default(); M];

        for i in 0..M {
            ret[i] = Key::new(n[i], d[i])
        }

        ret
    }
}

