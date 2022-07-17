use std::marker::PhantomData;
use crate::tty::KeySetRes;


pub trait Node: Copy + Clone + Default + Eq + PartialEq {}


pub trait Key:  Copy + Clone + Default + Eq + PartialEq {

}


pub trait Ledger: Copy + Clone + Default + Eq + PartialEq {}


