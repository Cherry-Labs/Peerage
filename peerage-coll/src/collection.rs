use crate::array_holder::ArrayHolder;
use std::{ops::{Index, IndexMut, Add, Sub, Mul, Div, Rem}, iter};
use crate::res::*;

#[derive(Clone, Copy, Default)]
pub struct PeerageCollection<T: Clone + Copy + Default + Default> {
    array: ArrayHolder<T>,
    current_buffer: ArrayHolder<T>,
    filler_buffer: usize,
    indexer: isize,
    curr_size: usize,

}

impl<T: Clone + Copy + Default + Default> PeerageCollection<T> {
    pub fn default() -> Self {
        let array = ArrayHolder::Init([T::default(); 0]);        
        let current_buffer = ArrayHolder::I0([T::default(); 1024]);
        let filler_buffer = 0usize;
        let indexer = -1isize;
        let curr_size = 0usize;

        Self {
            array,
            current_buffer, 
            filler_buffer,
            indexer,
            curr_size          
        }
    }

    pub fn new() -> Self {
        Self::default()
    }
   
    pub fn from_vector(v: Vec<T>) -> Self {
        if v.len() > 17408 {
            panic!("Length larger than maximum");
        }
        
        let mut default = Self::default();

        for m in v {
            default.push(m);
        }

        default

    }

    pub fn new_i0_from_item(item: T) -> Self {
        let array = ArrayHolder::I0([item; 1024]);
        let current_buffer = ArrayHolder::I0([T::default(); 1024]);
        let filler_buffer = 0usize;
        let indexer = -1isize;
        let curr_size = 0usize;

        Self { 
                array,
                current_buffer, 
                filler_buffer,
                indexer, 
                curr_size
            }

    }

    pub fn len(&self) -> usize {
        self.filler_buffer.clone()
    }
    
    pub fn default_init_size(size: usize) -> Self {
        let v = vec![T::default(); size];

        Self::from_vector(v)
    }

    pub fn contains(&self, t: T) -> bool {
        let mut self_iter = self.into_iter();

        for _ in 0..self_iter.clone().count() {
            let item = self_iter.next().unwrap();

            match item {
                t => return true,
                _ => continue
            }
        }

        false
    }

    pub fn into_vec(&self) -> Vec<T> {
        let mut v: Vec<T> = vec![];

        for i in 0..self.len() {
            let item = self.get_at(i).unwrap();
            v.push(item);
        }

        v
    }

    pub fn default_and_replace(
        &mut self, buffer: Vec<T>, size: usize) {
        
        let mut default_array = vec![T::default(); size];
        
        let self_array = self.array.unwrap();

        for i in 0..self.curr_size {
           default_array[i] = self_array[i]
        }

        let l = self.curr_size;

        for (i, t) in buffer.into_iter().enumerate() {
            default_array[l + i] = t;
        }
       
        let default_slice = default_array.as_slice();

        self.array.rewrap(default_slice);
    }

       

    pub fn set_at(&mut self, index: usize, value: T) -> SetResultType {
        let result = match index > self.curr_size {
            true => {
                match index < self.filler_buffer {
                    true => {
                        self.current_buffer[self.filler_buffer - index] = value;
                        Ok(SetResult::Success)   
                    },
                    false => Err(SetResult::Failure),
                }
            },
            false => {
                self.array[index] = value;
                Ok(SetResult::Success)
            },
        };

        result        
    }

    pub fn get_at(&self, index: usize) -> Option<T> {
        match index > self.curr_size {
            true => {
                match index < self.filler_buffer  {
                    true => Some(self.current_buffer[self.filler_buffer - index]),
                    false => None,
                }
            },
            false => Some(self.array[index]),
        }
    }


    pub fn get_at_mut<'a>(&'a mut self, index: usize, rep: &'a mut T) {
        match index > self.curr_size {
            true => {
                match index < self.filler_buffer  {
                    true => {
                        let mut item = self.current_buffer[self.filler_buffer - index];
                        
                        *rep  = item;
                    },
                    false => {
                        let mut item = T::default();

                        *rep = item;
                    },
                }
            },
            false => {
                let mut item = self.array[index];

                *rep = item;
            },
        }
    }


    pub fn push(&mut self, item: T) {
        if self.filler_buffer % 1024 != 0 {
            let ind = self.filler_buffer % 1024;

            self.current_buffer[ind] = item;
            
            self.filler_buffer += 1;
            self.indexer = self.filler_buffer as isize % 1024;

        } else {           
            self.filler_buffer += 1;           
            let buffer = self.current_buffer.unwrap().to_vec();
            self.default_and_replace(buffer, self.filler_buffer);
            self.current_buffer = ArrayHolder::I0([T::default(); 1024]);
            self.push(item);
        }
    }

    pub fn insert_all_into(
        &mut self, 
        old: Self,
        old_indices: Vec<isize>, 
        new_indices: Vec<isize>,
    ) {
        for (oldi, newi) in old_indices
                                            .into_iter()
                                           .zip(new_indices.into_iter())
            
        {
            if oldi == -1 || newi == -1 {
                continue;
            }

            let old_item = old[oldi as usize];

            self[newi as usize] = old_item;

        }
    }

    pub fn take_size_to_next_level(
        &mut self, 
        t: T, 
        old_indices: Vec<isize>, 
        new_indices: Vec<isize>,
    ) {
        match self.array {
            ArrayHolder::Empty => {
                let mut self_old = self.clone();
                self.array = ArrayHolder::I0([t; 1024]);
                self.insert_all_into(self_old, old_indices, new_indices);

            },            
            ArrayHolder::Init(_) => {
                let mut self_old = self.clone();
                self.array = ArrayHolder::I0([t; 1024]);
                self.insert_all_into(self_old, old_indices, new_indices);

            },            
            ArrayHolder::I0(_) => {
                let mut self_old = self.clone();
                self.array = ArrayHolder::I1([t; 2048]);
                self.insert_all_into(self_old, old_indices, new_indices);

            },            
            ArrayHolder::I1(_) => {
                let mut self_old = self.clone();
                self.array = ArrayHolder::I2([t; 3072]);
                self.insert_all_into(self_old, old_indices, new_indices);

            },
            ArrayHolder::I2(_) => {
                let mut self_old = self.clone();
                self.array = ArrayHolder::I3([t; 4096]);
                self.insert_all_into(self_old, old_indices, new_indices);

            },
            ArrayHolder::I3(_) => {
                let mut self_old = self.clone();
                self.array = ArrayHolder::I4([t; 5120]);
                self.insert_all_into(self_old, old_indices, new_indices);

            },
            ArrayHolder::I4(_) => {
                let mut self_old = self.clone();
                self.array = ArrayHolder::I5([t; 6144]);
                self.insert_all_into(self_old, old_indices, new_indices);

            },
            ArrayHolder::I5(_) => {
                let mut self_old = self.clone();
                self.array = ArrayHolder::I6([t; 7168]);
                self.insert_all_into(self_old, old_indices, new_indices);

            },
            ArrayHolder::I6(_) => {
                let mut self_old = self.clone();
                self.array = ArrayHolder::I7([t; 8192]);
                self.insert_all_into(self_old, old_indices, new_indices);

            },
            ArrayHolder::I7(_) => {
                let mut self_old = self.clone();
                self.array = ArrayHolder::I8([t; 10240]);
                self.insert_all_into(self_old, old_indices, new_indices);

            },
            ArrayHolder::I8(_) => {
                let mut self_old = self.clone();
                self.array = ArrayHolder::I9([t; 11264]);
                self.insert_all_into(self_old, old_indices, new_indices);

            },
            ArrayHolder::I9(_) => {
                let mut self_old = self.clone();
                self.array = ArrayHolder::I10([t; 12288]);
                self.insert_all_into(self_old, old_indices, new_indices);

            },
            ArrayHolder::I10(_) => {
                let mut self_old = self.clone();
                self.array = ArrayHolder::I11([t; 133128]);
                self.insert_all_into(self_old, old_indices, new_indices);

            },
            ArrayHolder::I11(_) => {
                let mut self_old = self.clone();
                self.array = ArrayHolder::I12([t; 14336]);
                self.insert_all_into(self_old, old_indices, new_indices);

            },
            ArrayHolder::I12(_) => {
                let mut self_old = self.clone();
                self.array = ArrayHolder::I13([t; 15360]);
                self.insert_all_into(self_old, old_indices, new_indices);

            },
            ArrayHolder::I13(_) => {
                let mut self_old = self.clone();
                self.array = ArrayHolder::I14([t; 16384]);
                self.insert_all_into(self_old, old_indices, new_indices);

            },
            ArrayHolder::I14(_) => {
                let mut self_old = self.clone();
                self.array = ArrayHolder::I15([t; 17408]);
                self.insert_all_into(self_old, old_indices, new_indices);

            },
            ArrayHolder::I15(_) => {
                let mut self_old = self.clone();
                self.array = ArrayHolder::I15([t; 17408]);
                self.insert_all_into(self_old, old_indices, new_indices);

            },
        }
     
    }

    pub fn all_occupied(&self, def: T) -> bool {
        for i in 0..self.len() {
            match self[i] {
                def => return false,
                _ => continue,
            }
        
        }

        true
    }
    
}


impl<T: Clone + 
        Copy + 
        Default 
    > std::ops::Index<usize> for PeerageCollection<T> {
        type Output = T;

        fn index(&self, index: usize) -> &Self::Output {
            match index > self.array.unwrap().len() {
                true => &self.current_buffer[index % 1024],
                false => &self.array[index],
            }
        }
    }


impl<T: Clone +
            Copy +
            Default
        > std::ops::IndexMut<usize> for PeerageCollection<T> {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                match index > self.array.unwrap().len() {
                    true => &mut self.current_buffer[index % 1024],
                    false => &mut self.array[index],
                }
            }
        }


impl<T: Clone + Copy + Default> From<Vec<T>> for PeerageCollection<T> {
    fn from(v: Vec<T>) -> Self {
        Self::from_vector(v)
    }
}

impl<T: Clone + Copy + Default> Into<Vec<T>> for PeerageCollection<T> {
    fn into(self) -> Vec<T> {
        self.into_vec()
    }
}