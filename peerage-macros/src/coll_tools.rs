use std::str::Chars;

use proc_macro2;
use quote::quote;

pub fn parse_and_create_ts(s: String) -> proc_macro2::TokenStream {
    let s_rem_q = s.replace("\"", "");

    let ts = match s_rem_q.contains(";") {
        true => {
            let split = s_rem_q.split(";");

            let tty = split.next().unwrap().trim();
            let size = split.next().unwrap().trim();

            if size.chars().next().unwrap().is_alphabetic() {
                panic!("Wrong size: '{}'", size);
            }

            

        },
        false => todo!(),
    };
}