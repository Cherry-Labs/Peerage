use quote::{quote, format_ident};
use proc_macro2::TokenStream;

fn make_arr_list(tty: String, num: usize) -> TokenStream {
    let vec_str = vec![format_ident!("{}", tty); num];

    quote! {
       {
            [
                #(#vec_str),*
            ]
       }
    }
}


fn make_ve_list(tty: String, num_var: String, insert: String) -> TokenStream {
    quote! {
       {
            {
                let mut vec: Vec<#tty> = vec![];

                for _ in 0..#num_var {
                    vec.push(#insert);
                }

                vec
            }
       }
    }
}

pub fn vec_main(s: String) -> TokenStream {
    let remq = s.replace("\"", "");

    let mut split = remq.split(";");

    let tty = split.next().unwrap().trim().to_string();
    let num_var = split.next().unwrap().trim().to_string();
    let insert = split.next().unwrap().trim().to_string();


    make_ve_list(tty, num_var, insert)
}



pub fn array_main(str: String) -> TokenStream {
    let str_wq = str.replace("\"", "");

    let mut str_split = str_wq.split(";");

    let tty = str_split.next().unwrap().trim();
    let num_str = str_split.next().unwrap().trim();

    let num = num_str.parse::<usize>().expect(&format!("Wrong num: '{}'", num_str));


    make_arr_list(tty.to_string(), num)

}


