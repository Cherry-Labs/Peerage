use crate::array_holder::ArrayHolder;
use std::ops::{Index, IndexMut, Add, Sub, Mul, Div, Rem};
use crate::res::*;
use peerage_macros::make_arr;

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

    pub fn len(&self) -> usize {
        self.filler_buffer.clone()
    }
    
    pub fn default_init_size(size: usize) -> Self {
        let v = vec![T::default(); size];

        Self::from_vector(v)
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
    
}
