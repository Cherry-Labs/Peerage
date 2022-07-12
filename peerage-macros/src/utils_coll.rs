use std::str::FromStr;

use proc_macro2;
use quote::{quote, quote_spanned, format_ident};
use syn::spanned::Spanned;


enum Command {
    Selfer,
    Refer,
    Muter,
}

impl Command {
    pub fn from_str(str: &str) -> Self {
        match str {
            "self" => Self::Selfer,
            "ref" => Self::Refer,
            "mut" => Self::Muter,
            _ => panic!("Command can either be self, mut, or ref not {}", str),
        }
    }

    pub fn get_func_name(&self, num: usize, holder_name: proc_macro2::Ident) -> proc_macro2::TokenStream {
        let func_name = crate::global_consts::NAME_TABLE.get(&num).unwrap();

        let ret_item = match self {
            Command::Selfer => func_name.replace("*", "self"),
            Command::Refer => func_name.replace("*", "ref"),
            Command::Muter => func_name.replace("*", "mut"),
        };

        let name_item = ret_item.replace("\"", "");

        let ident_name = format!("{}()", name_item);

        let ident_joined = format!("{}.coll_wrapper.{}", holder_name, ident_name);

        let ts = proc_macro2::TokenStream::from_str(&ident_joined).unwrap();


        quote!( #ts )
    }
}


pub struct BlockQuadruplet {
    holder_name: proc_macro2::Ident,
    command: Command,
    size: usize,
    block_content: proc_macro2::TokenStream,

}


impl BlockQuadruplet  {
    pub fn new_and_parse(s: String) -> Self {
        let snq = s.replace("\"", "");

        let holder_name = Self::get_holder_name(snq.clone());
        let command = Self::get_command_ts(snq.clone());
        let size = Self::get_holder_size(snq.clone());
        let block_content = command.get_func_name(size, holder_name.clone());


        Self { holder_name, command, size, block_content }
    }

    fn get_holder_name(input_str: String) -> proc_macro2::Ident {
        let mut name_chars: Vec<char> = vec![];

        let mut do_insert = false;

        for c in input_str.chars() {
            if c == '/' && do_insert {
                break;
            }
            
            if do_insert {
                name_chars.push(c);
            }            

            if c == '"' || c == ' ' {
                continue;
            }

            if c == '%' {
                do_insert = true;
            }
        }

        let str_fin = String::from_iter(name_chars);

        format_ident!("{}", str_fin)
    }

    fn get_command_ts(input_str: String) -> Command {
        let mut name_chars: Vec<char> = vec![];

        let mut do_insert = false;

        for c in input_str.chars() {
            if c == '/' && do_insert {
                break;
            }

            if do_insert {
                name_chars.push(c);
            }

            if c == '"' || c == ' ' {
                continue;
            }

            if c == '*' {
                do_insert = true;
            }
        }

        let str_fin = String::from_iter(name_chars);

        Command::from_str(&str_fin)
    }


    fn get_holder_size(input_str: String) -> usize {
        let mut name_chars: Vec<char> = vec![];

        let mut do_insert = false;

        for c in input_str.chars() {
            if c == '/' && do_insert {
                break;
            }
            
            if do_insert {
                name_chars.push(c);
            }

            if c == '"' || c == ' ' {
                continue;
            }

            if c == '$' {
                do_insert = true;
            }
        }

        let str_fin = String::from_iter(name_chars);

        let num = str_fin.trim().parse::<usize>().expect(&format!("Not {}", str_fin));

        if !vec![2usize, 4, 16, 256, 512, 1024, 2048, 4096].contains(&num) {
            panic!("Num should be 2, 4, 16, 256, 512, 1024, 2048, 4096")
        } else {
            num
        }
    }
    
    pub fn get_block(&self) -> proc_macro2::TokenStream {
        self.block_content.clone()
    }

}