#![allow(unused)]

pub mod quadrupleword;
pub mod byte;
pub mod nibble;
pub mod byteword;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
