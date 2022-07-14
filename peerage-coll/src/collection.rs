use crate::array_holder::ArrayHolder;
use peerage_utils::traits::Node;
use std::ops::{Index, IndexMut, Add, Sub, Mul, Div, Rem};


#[derive(Clone, Copy)]
pub struct PeerageCollection<T: Copy + Clone + Node> {
    array: ArrayHolder<T>,
    current_buffer: [T; 1024],
    filler_buffer: usize,
    indexer: isize,
    curr_size: usize,

}

impl<T: Copy + Clone + Node> PeerageCollection<T> {
    pub fn new() -> Self {
        let array = ArrayHolder::Init([T::new(); 0]);        
        let current_buffer = [T::new(); 1024];
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
        
        let mut new = Self::new();

        for m in v {
            new.push(m);
        }

        new

    }

    pub fn len(&self) -> usize {
        self.filler_buffer.clone()
    }
    
    pub fn new_init_size(size: usize) -> Self {
        let v = vec![T::new(); size];

        Self::from_vector(v)
    }

    pub fn new_and_replace(
        &mut self, buffer: Vec<T>, size: usize) {
        
        let mut new_array = vec![T::new(); size];
        
        let self_array = self.array.unwrap();

        for i in 0..self.curr_size {
           new_array[i] = self_array[i]
        }

        let l = self.curr_size;

        for (i, t) in buffer.into_iter().enumerate() {
            new_array[l + i] = t;
        }
       
        let new_slice = new_array.as_slice();

        self.array.rewrap(new_slice);
    }

       

    pub fn set_at(&mut self, index: usize, value: T) {
        match index > self.curr_size {
            true => {
                match index < self.filler_buffer {
                    true => self.current_buffer[self.filler_buffer - index] = value,
                    false => panic!("Large index! Index is {} whilst length is: {}", index, self.curr_size + 1024),
                }
            },
            false => self.array[index] = value,
        }
    }

    pub fn get_at(&self, index: usize) -> T {
        match index > self.curr_size {
            true => {
                match index < self.filler_buffer  {
                    true => self.current_buffer[self.filler_buffer - index],
                    false => panic!("Large index!"),
                }
            },
            false => self.array[index],
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
            let buffer = self.current_buffer.to_vec();
            self.new_and_replace(buffer, self.filler_buffer);
            self.current_buffer = [T::new(); 1024];
            self.push(item);
        }
    }
    
}

impl<T: Clone + Copy + Node> Index<usize> for PeerageCollection<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.array[index]
    }
}


impl<T: Clone + Copy + Node> IndexMut<usize> for PeerageCollection<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.array[index]

    }
}

impl<T: Clone + Copy + Node> Add for PeerageCollection<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut self_clone_mut = self.clone();
        
        self_clone_mut.array = self.array + rhs.array;

        self_clone_mut
    }
}

impl<T: Clone + Copy + Node> Mul for PeerageCollection<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut self_clone_mut = self.clone();
        
        self_clone_mut.array = self.array * rhs.array;

        self_clone_mut
    }
}

impl<T: Clone + Copy + Node> Sub for PeerageCollection<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut self_clone_mut = self.clone();
        
        self_clone_mut.array = self.array - rhs.array;

        self_clone_mut
    }
}


impl<T: Clone + Copy + Node> Div for PeerageCollection<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let mut self_clone_mut = self.clone();
        
        self_clone_mut.array = self.array / rhs.array;

        self_clone_mut
    }
}

impl<T: Clone + Copy + Node> Rem for PeerageCollection<T> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        let mut self_clone_mut = self.clone();
        
        self_clone_mut.array = self.array % rhs.array;

        self_clone_mut
    }
}