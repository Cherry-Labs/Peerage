use std::{marker::PhantomData, borrow::BorrowMut, clone};
use safe_transmute::{SingleManyGuard, transmute_many_mut};
use peerage_utils::traits::Node;

#[derive(Clone, Copy)]
pub struct PeerageCollection<T: Copy + Clone + Node, const M: usize> {
    array: [T; M],
    current_buffer: [T; 1024],

}

impl<T: Copy + Clone + Node, const M: usize> PeerageCollection<T, M> {
    pub fn new() -> Self {
        let array = [T::new(); M];        
        let current_buffer = [T::new(); 1024];
       

        Self {
            array,
            current_buffer,           
        }
    }

    pub fn new_and_replace<C: Clone + Copy + Node, 
    const N: usize, const D: usize>(
        &mut self, to_replace: [T; D], buffer: Vec<T>) {
        let mut new = PeerageCollection::<C, N>::new();

        for i in 0..to_replace.len() {
            let at = to_replace[i];
            let conv = T::into_self::<T, C>(at, PhantomData);

            new.set_at(i, conv);
        }

        let l = to_replace.len();

        for (i, t) in buffer.into_iter().enumerate() {
            let conv = T::into_self::<T, C>(t, PhantomData);

            new.set_at(l + i, conv);
        }
       

        
    }

       

    pub fn set_at(&mut self, index: usize, value: T) {
        self.array[index] = value;
    }

    pub fn get_at(&self, index: usize) -> &T {
        &self.array[index]
    }

    pub fn load_into_buffer(&mut self, start: usize, end: usize) {
        if (end - start) - 8 > 0 {
            panic!("Range higher than 8")
        };
        if end > (1024 - 8) {
            panic!("End larger than 1024 - 8")
        };

        let range = &self.array[start..end];

        for i in 0..8 {
            self.current_buffer[i] = range[i].clone();
        }
    }

    
}

impl<T: Copy + Clone + Node, const M: usize> std::ops::Index<usize>
    for PeerageCollection<T, M>
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index > 1023 {
            true => panic!("Size of BPTreeCollection is 1024. Index ends at 1023."),
            false => self.get_at(index),
        }
    }
}


pub struct PeerageCollectionWrapper<T: Copy + Clone + Node> {
    collection: PeerageCollection<T, 0>,
    filler_buffer: usize,
    indexer: isize,
}

impl<T: Copy + Clone + Node> PeerageCollectionWrapper<T> {
    pub fn new() -> Self {
        let collection = PeerageCollection::<T, 0>::new();

        let filler_buffer = 1usize;
        let indexer = -1isize;

        Self { collection, filler_buffer, indexer }
    }

    pub fn get_at(&self, index: usize) -> &T {
        self.collection.get_at(index)
    }

    pub fn set_at(&mut self, index: usize, object: T) {
        self.collection.set_at(index, object)
    } 

    pub fn new_updated_self(&mut self, buffer: Vec<T>) {
        match self.indexer {
            0 => {
                const M: usize = 1024;
                
               let new_self = PeerageCollection::<T, M>::new_and_replace(self.array, buffer);
               
               let coll_transmute_copy: PeerageCollection<T, M> = unsafe { std::mem::transmute_copy(&new_self) };
               
               
            },
            1 => {
                const M: usize = 2048;

                let new_self = PeerageCollection::<T, M>::new_and_replace(self.array, buffer);
                
                self = unsafe {
                    std::mem::transmute::<Self, PeerageCollection<T, M>>() 
               };
             },
            2 => {
                const M: usize = 3072;

            
                let new_self = PeerageCollection::<T, M>::new_and_replace(self.array, buffer);
                
                self = unsafe {
                    std::mem::transmute::<Self, PeerageCollection<T, M>>() 
               };
             },
            3 => {
                const M: usize = 4096;
    
        
                let new_self = PeerageCollection::<T, M>::new_and_replace(self.array, buffer);
                
                self = unsafe {
                    std::mem::transmute::<Self, PeerageCollection<T, M>>() 
               };
             },
            4 => {
                let new_self = PeerageCollection::<T, 5120>::new_and_replace(self.array, buffer);
                
                self = unsafe {
                    std::mem::transmute::<Self, PeerageCollection<T, 5120>>() 
               };
             },
            5 => {
                const M: usize = 6144;
    
        
                let new_self = PeerageCollection::<T, M>::new_and_replace(self.array, buffer);
                
                self = unsafe {
                    std::mem::transmute::<Self, PeerageCollection<T, M>>() 
               };            

             },
            6 => {
                const M: usize = 7168;
    
        
                let new_self = PeerageCollection::<T, M>::new_and_replace(self.array, buffer);
                
                self = unsafe {
                    std::mem::transmute::<Self, PeerageCollection<T, M>>() 
               }; 

             },
            7 => {
                let new_self = PeerageCollection::<T, 8192>::new_and_replace(self.array, buffer);
                
                self = unsafe {
                    std::mem::transmute::<Self, PeerageCollection<T, 8192>>() 
               };
             },
            8 => {
                const M: usize = 8192;
    
        
                let new_self = PeerageCollection::<T, M>::new_and_replace(self.array, buffer);
                
                self = unsafe {
                    std::mem::transmute::<Self, PeerageCollection<T, M>>() 
               }; 
             },
            9 => {
                const M: usize = 10240;
    
        
                let new_self = PeerageCollection::<T, M>::new_and_replace(self.array, buffer);
                
                self = unsafe {
                    std::mem::transmute::<Self, PeerageCollection<T, M>>() 
               }; 
             },
            10 => {
                const M: usize = 11264;
    
        
                let new_self = PeerageCollection::<T, M>::new_and_replace(self.array, buffer);
                
                self = unsafe {
                    std::mem::transmute::<Self, PeerageCollection<T, M>>() 
               }; 
             },
            11=> {
                const M: usize = 12288;
    
        
                let new_self = PeerageCollection::<T, M>::new_and_replace(self.array, buffer);
                
                self = unsafe {
                    std::mem::transmute::<Self, PeerageCollection<T, M>>() 
               }; 
             },
            12 => {
                const M: usize = 133128;
    
        
                let new_self = PeerageCollection::<T, M>::new_and_replace(self.array, buffer);
                
                self = unsafe {
                    std::mem::transmute::<Self, PeerageCollection<T, M>>() 
               }; 
             },
             11 => {
                const M: usize = 14336;
    
        
                let new_self = PeerageCollection::<T, M>::new_and_replace(self.array, buffer);
                
                self = unsafe {
                    std::mem::transmute::<Self, PeerageCollection<T, M>>() 
               }; 

             },
            13 => {
                const M: usize = 15360;
    
        
                let new_self = PeerageCollection::<T, M>::new_and_replace(self.array, buffer);
                
                self = unsafe {
                    std::mem::transmute::<Self, PeerageCollection<T, M>>() 
               }; 
             },
            14 => {
                const M: usize = 16384;
    
        
                let new_self = PeerageCollection::<T, M>::new_and_replace(self.array, buffer);
                
                self = unsafe {
                    std::mem::transmute::<Self, PeerageCollection<T, M>>() 
               }; 
             },
            15 => {
                const M: usize = 17408;
    
        
                let new_self = PeerageCollection::<T, M>::new_and_replace(self.array, buffer);
                
                self = unsafe {
                    std::mem::transmute::<Self, PeerageCollection<T, M>>() 
               }; 
             },
        }
    }

    pub fn push(&mut self, item: T) {
        println!("{}", self.filler_buffer % 1024 );
    
        if self.filler_buffer % 1024 != 0 {
            let ind = self.filler_buffer % 1024;

            self.collection.current_buffer[ind] = item;
            
            self.filler_buffer += 1;
        } else {
            self.indexer = self.filler_buffer as isize / 1024;

            if self.indexer == 16 {
                self.indexer = 0isize;
            }
            self.filler_buffer += 1;
           
            let buffer_vec = self.collection.current_buffer.to_vec();

            let new_self = Self::new_updated_self(self, self.array, buffer_vec);

            std::mem::replace(self, new_self);

            self.collection.current_buffer = [T::new(); 1024];

            self.push(item);



        }
    }

}