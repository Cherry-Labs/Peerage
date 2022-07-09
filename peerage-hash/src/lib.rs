#![allow(unused)]

mod rounder;
mod convert_utils;
pub mod hasher;
mod hash_bin;

#[macro_export]
macro_rules! scooch_over {
        ($a_rep: ident, $b_rep: ident, $c_rep: ident) => {
            (
                $a_rep, $b_rep, $c_rep
            ) = {
                (
                    $a_rep,
                    $b_rep,
                    $c_rep,
                )
            }
        };
    }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
