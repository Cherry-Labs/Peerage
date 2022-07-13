use std::str::{Chars, FromStr};

use proc_macro2::{self, TokenStream};
use quote::quote;

pub fn parse_and_create_ts(s: String) -> proc_macro2::TokenStream {
    let s_rem_q = {
        let mut chars = s.chars();

        let first  = chars.clone().next().unwrap();
        let last = chars.clone().last().unwrap();

        let mut f = 0usize;
        let mut l = s.len() - 1;

        if first == '"' || first == '\'' {
            f = 1;
        }

        if last == '"' || last == '\'' {
            l = s.len() - 2;
        }

        let new_string = &s[f..l];

        new_string

    };

    let ts = match s_rem_q.contains(";") {
        true => {
            let mut split = s_rem_q.split(";");

            let obj = split.next().unwrap().trim();
            let size = split.next().unwrap().trim();

            if size.chars().next().unwrap().is_alphabetic() {
                panic!("Wrong size: '{}'", size);
            }

            let function_call = format!("peerage_coll::collection::PeerageCollection::from_vector(vec![{obj}, {size}])");

            let ts = proc_macro2::TokenStream::from_str(&function_call).unwrap();

            quote! {
                {
                    #ts
                }
            }

        },
        false => {
            let mut split = s_rem_q.split("//");

            let len = split.clone().count();

            let tty = split.next().expect(&format!("lent is {len}")).trim();
            let objects = split.next().expect(&format!("lent is {len}")).trim();

            let objects_split = objects.split(",");

            if objects_split.clone().count() == 0 {
                panic!("Length is 0")
            }


            let mut interplate_ts: Vec<proc_macro2::TokenStream>  = vec![];

            for os in objects_split {
                let push_str = format!("coll.push({os});");
                let ts = proc_macro2::TokenStream::from_str(&push_str).unwrap();

                interplate_ts.push(ts);

            }

            let new_obj = format!("let coll = peerage_coll::collection::PeerageCollection::<{tty}>();");

            let new_obj_ts = proc_macro2::TokenStream::from_str(&new_obj).expect(&format!("Type is {tty}"));

            quote! {
            
                {
                    #new_obj_ts
                    #(#interplate_ts)*


                    coll
                }


            }
        },
    };


    ts
}

