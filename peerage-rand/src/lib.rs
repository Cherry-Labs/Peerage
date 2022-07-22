#![allow(unused)]

mod quadrupleword;
mod byte;
mod nibble;
mod byteword;
pub mod rand;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
