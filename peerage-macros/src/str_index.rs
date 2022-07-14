use std::str::FromStr;

use proc_macro2;
use quote::quote;


fn add_to_index(add: String, name: String) -> proc_macro2::TokenStream {
    let add_this = "{ format!(";

    let added = "{}{}\", self.curr_index,  ";
    
    let fin = format!("{}{}\"{}\") }}", add_this, added, add);

    let fin_ts = proc_macro2::TokenStream::from_str(&fin).unwrap();

    fin_ts
}


pub fn parse_and_get_index(str: String) -> proc_macro2::TokenStream {
    let str_remqoute = str.replace("\"", "");

    let mut str_split = str_remqoute.split("//");

    let name = str_split.next().unwrap().to_string();
    let add = str_split.next().unwrap().to_string();

    let ts = add_to_index(add, name);

    quote! {
        #ts
    }
}