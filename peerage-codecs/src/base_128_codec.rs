use peerage_utils::bin_utils::QuadrupleWord;
use crate::constant_values::BASE_128_STRS;


pub struct Base128Codec<const M: usize>([QuadrupleWord; M]);


impl<const M: usize> Base128Codec<M> {
    fn new(arr: [QuadrupleWord; M]) -> Self {
        Base128Codec(arr)
    }

    fn unwrap(&self) -> [QuadrupleWord; M] {
        let Base128Codec(item) = *self;

        item
    }

    fn encode_inner(&self) -> String {
        let mut fin = String::new();
        
        let qw_128 = QuadrupleWord::from_u128(128);

        let item = self.unwrap();

        for i in 0usize..m {
            let modded = item[i] % qw_128;

            let str_mod128 = BASE_128_STRS[&modded];


            fin = format!("{}{}", fin, str_mod128);
        }
        

        fin
    }
    
    


}