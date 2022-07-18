use std::marker::PhantomData;
use crate::tty::KeySetRes;


pub trait Node: Copy + Clone + Default + Eq + PartialEq {}


pub trait Key:  Copy + Clone + Default + Eq + PartialEq {
    fn get_key_index(&self, size: usize) -> usize;
}


pub trait Ledger: Copy + Clone + Default + Eq + PartialEq {}



