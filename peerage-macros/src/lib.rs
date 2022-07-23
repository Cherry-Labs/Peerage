#![allow(unused)]

extern crate proc_macro;

mod coll;

use proc_macro::TokenStream;
use coll::coll_parse;
use quote::ToTokens;

#[proc_macro]
pub fn coll(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();

    let ts = coll_parse(input_str);

    ts.into()
}