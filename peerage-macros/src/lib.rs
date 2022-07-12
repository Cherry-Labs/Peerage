#![allow(unused)]

#[macro_use]
extern crate lazy_static;

mod utils_coll;
mod global_consts;
mod interpolate_tools;
use proc_macro::TokenStream;
use crate::utils_coll::BlockQuadruplet;



#[proc_macro]
pub fn block_out_return_holder(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();

    let quadruplet = BlockQuadruplet::new_and_parse(input_str);

    let fin_ts_v2 = quadruplet.get_block();


    fin_ts_v2.into()
}