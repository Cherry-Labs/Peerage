#![allow(unused)]

mod quadruple_word;
mod byte;
mod nibble;
mod byte_word;
mod sesset;
mod double_word;
pub mod rand;
pub mod zero_eighty;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
