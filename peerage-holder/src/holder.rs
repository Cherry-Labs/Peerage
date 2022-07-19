
use std::ptr;

#[derive(Clone, Copy)]
pub enum Holder<T: Copy + Default + Clone> {
    Nil,
    Refer(*const T),
    Mutter(*mut T)
}

impl<T: Copy + Default + Clone> Default for Holder<T> {
    fn default() -> Self {
        Self::Nil
    }
}

impl<T: Copy + Default + Clone> Holder<T> {
    pub fn refer_from_item(item: T) -> Self {
        let item_ptr = ptr::addr_of!(item);
    
        Holder::Refer(item_ptr)
    }

    pub fn get_refer_ref(&self) -> Option<&T> {
        match self {
            Holder::Nil => None,
            Holder::Refer(item) => {
                let item_deref = unsafe {
                    item.as_ref()
                };

                item_deref
            },
            Holder::Mutter(_) => None,
        }
    }

    pub fn mutter_from_item(item: T) -> Self {
        let mut item_mut = item.clone();

        let item_ptr = ptr::addr_of_mut!(item_mut);
    
        Holder::Mutter(item_ptr)
    }

    pub fn get_mutter_ref(&self) -> Option<&mut T> {
        match self {
            Holder::Nil => None,
            Holder::Refer(_) => None,
            Holder::Mutter(item) => {
                let item_deref = unsafe {
                    item.as_mut()
                };

                item_deref
            },
        }
    }
}