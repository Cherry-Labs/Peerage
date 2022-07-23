use std::str::FromStr;

use proc_macro2::TokenStream;
use quote::quote;


fn coll_repeat(item: String, size: usize) -> TokenStream {
    let item_repeat = vec![TokenStream::from_str(&format!("v.push({item});")).unwrap(); size - 1];

    let item_begin = TokenStream::from_str(&format!("let mut v = vec![{item}];")).unwrap();

    quote! {
        {
            #item_begin

            #(#item_repeat)*

            peerage_coll::collection::PeerageCollection::from_vector(v)
        }
    }
}

fn coll_insert(items: String) -> TokenStream {
    let vec_items = items
                        .split(",")
                        .enumerate()
                        .filter(|(u, _)| *u > 1)
                        .map(|(_, x)| TokenStream::from_str(&format!("v.push({});", x.trim())).unwrap())
                        .collect::<Vec<TokenStream>>();

    let mut items_split = items.split(",");

    let first_item = items_split.next().unwrap().trim();

    let item_begin = TokenStream::from_str(&format!("let mut v = vec![{first_item}];")).unwrap();


    quote! {
        {
            #item_begin

            #(#vec_items)*

            peerage_coll::collection::PeerageCollection::from_vector(v)
        }
    }
}


pub fn coll_parse(swq: String) -> TokenStream {
    let s = swq.replace("\"", "");
    
    match s.contains(";") {
        true => {
            let mut s_split = s.split(";");
            
            let item = s_split.next().unwrap().trim();
            let num_str = s_split.next().unwrap().trim();

            let size = num_str.parse::<usize>().expect(&format!(" wrong num str '{}'", num_str));
            
            coll_repeat(item.to_string(), size)
        },
        false => coll_insert(s),
    }
}