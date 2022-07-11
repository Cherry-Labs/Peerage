#![allow(unused)]

mod utils_coll;


use proc_macro::TokenStream;
use crate::utils_coll::BlockQuadruplet;



#[proc_macro]
pub fn block_out_return(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();

    let quadruplet = BlockQuadruplet::new_and_parse(input_str);

    let fin_ts_v2 = quadruplet.get_block();


    fin_ts_v2.into()
}