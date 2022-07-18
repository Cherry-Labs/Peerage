use std::marker::PhantomData;
use crate::tty::KeySetRes;


pub trait Node: Copy + Clone + Default + Eq + PartialEq {}


pub trait Key:  Copy + Clone + Default + Eq + PartialEq {
    pub fn geT_key_index(&self) -> usize;
}


pub trait Ledger: Copy + Clone + Default + Eq + PartialEq {}



