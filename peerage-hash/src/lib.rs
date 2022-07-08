#![allow(unused)]

mod rounder;
mod convert_utils;
pub mod hasher;


#[macro_export]
macro_rules! scooch_over {
        ($a_rep: ident, $b_rep: ident, $c_rep: ident, $d_rep: ident, $e_rep: ident, $f_rep: ident, $g_rep: ident,$h_rep: ident) => {
            (
                $a_rep, $b_rep, $c_rep, $d_rep, $e_rep, $f_rep, $g_rep, $h_rep,
            ) = {
                (
                    $a_rep,
                    $b_rep,
                    $c_rep,
                    $d_rep,
                    $e_rep,
                    $f_rep,
                    $g_rep,
                    $h_rep,
                )
            };
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
