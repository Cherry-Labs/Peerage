use std::str::FromStr;

use proc_macro2;
use quote::quote;


pub fn make_array_list(n: String, t: String, len: usize) -> proc_macro2::TokenStream {
    let tty = proc_macro2::TokenStream::from_str(&t).expect(&format!("{} wrong", t));

    let mut name_tty_vec: Vec<proc_macro2::TokenStream> = vec![];

    for i in 0..len {
        let name_formatted = format!("{}_{}", n, i);
        let name = proc_macro2::TokenStream::from_str(&name_formatted).unwrap();

        let name_tty = quote!(#name: #tty,);

        
        name_tty_vec.push(name_tty)
    }


    quote! {
        #(#name_tty_vec)*
        length: usize,
    }

}   

pub fn signature(generics: String, name: String) -> proc_macro2::TokenStream  {
    let generics_ts = proc_macro2::TokenStream::from_str(&generics).expect(&format!("{} wrong", generics));
    let name_ts = proc_macro2::TokenStream::from_str(&name).expect(&format!("{} wrong", name));


    quote! {
        pub struct #name_ts<#generics_ts>
    }

}