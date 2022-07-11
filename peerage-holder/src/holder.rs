use std::borrow::{Borrow, BorrowMut};
use peerage_utils::pub_traits::Tree;



#[derive(Clone, Copy)]
pub enum Holder<'a, T: Clone + Copy + Tree> {
    Refer([&'a T; 1]),
    Selfer([T; 1]),
    Nil,
}


impl<'a, T: Clone + Copy + Tree> Holder<'a, T> {
    pub fn new_refer(refer: &'a T) -> Self {
        let array = [refer];

        Self::Refer(array)
    }

    pub fn new_selfer(selfer: T) -> Self {
        let array = [selfer];

        
        Self::Selfer(array)
    }
 
    
    pub fn unwrap(&self) -> Option<&T> {
        match self {
            Holder::Refer(refer) => {
                let selfer_clone = refer.clone();
                let item = selfer_clone[0];
                Some(item)
            },
            Holder::Selfer(selfer) => {
                let selfer_get = selfer.get(0).unwrap();
                
                Some(selfer_get)
            },
            Holder::Nil => None,
        }
    }


    pub fn is_container(&self) -> bool {
        match self {
            Holder::Refer(_) => true,
            Holder::Selfer(_) => true,
            Holder::Nil => false,
        }

    }

    pub fn is_nil(&self) -> bool {
        match self {
            Holder::Refer(_) => false,
            Holder::Selfer(_) => false,
            Holder::Nil => true,
        }

    }

    pub fn unwrap_no_ref(&self) -> Option<T> {
        let unwrapped = self.unwrap();

        let self_cloned = unwrapped.clone();

        match self.is_container() {
            true => {
                let self_unwrapped = self_cloned.unwrap();

                let self_clone = self_unwrapped.clone();

                Some(self_clone)
            },
            false => None
        }
    }

    pub fn replace(&mut self, with: Self) {
        let w_clone = with.clone();
        let mut s_clone = self.clone();

        std::mem::replace(s_clone.borrow_mut(), w_clone);
    }

   

}