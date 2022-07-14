#![allow(unused)]

#[macro_use]
extern crate lazy_static;

mod utils_coll;
mod global_consts;
mod coll_tools;
mod str_index;
use proc_macro::TokenStream;
use crate::utils_coll::BlockQuadruplet;
use coll_tools::parse_and_create_ts;
use str_index::parse_and_get_index;

#[proc_macro]
pub fn block_out_return_holder(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();

    let quadruplet = BlockQuadruplet::new_and_parse(input_str);

    let fin_ts_v2 = quadruplet.get_block();


    fin_ts_v2.into()
}

#[proc_macro]
pub fn coll(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();

    let fin_ts_v2 = parse_and_create_ts(input_str);


    fin_ts_v2.into()
}

#[proc_macro]
pub fn index_forward(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();

    let fin_ts_v2 = parse_and_get_index(input_str);


    fin_ts_v2.into()
}
