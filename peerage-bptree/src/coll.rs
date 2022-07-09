
use peerage_utils::pub_traits::Init;
use peerage_utils::bin_utils::QuadrupleWord;

pub struct BpTreeCollection<T: Init  + Copy + Clone, const B: usize, const M: usize> {
   array: [T; B],
   current_buffer: [T; M],
}

impl<T: Init  + Copy + Clone, const B: usize, const M: usize> BpTreeCollection<T, B, M>  {
    pub fn new() -> Self {
        let array = [T::init(); B];
        let current_buffer = [T::init(); M];

        Self { array, current_buffer }

    }


    pub fn from_array(array: [T; B]) -> Self {
        let current_buffer  = [T::init(); M];

        Self { array, current_buffer }
    }

    
    pub fn set_at(&mut self, index: usize, value: T) {
        self.array[index] = value;
    }

    pub fn get_at(&self, index: usize) -> &T {
        &self.array[index]
    }

    pub fn load_into_buffer(&mut self, start: usize, end: usize) {
        if (end - start) - 8 > 0 { panic!("Range higher than 8") };
        if end > (1024 - 8) { panic!("End larger than 1024 - 8") };
        
        
        let range = &self.array[start..end];

        for i in 0..8 {
            self.current_buffer[i] = range[i].clone();
        }

    }
}


impl<T: Init  + Copy + Clone, const B: usize, const M: usize>  std::ops::Index<usize> for BpTreeCollection<T, B, M> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index > 1023 {
            true => panic!("Size of BPTreeCollection is 1024. Index ends at 1023."),
            false => self.get_at(index),
        }
    }
}


pub struct BPTreeCollIterator<T: Init  + Copy + Clone, const B: usize, const M: usize> {
    coll: BpTreeCollection<T, B, M>,
    curr_index: usize
}


impl<T: Init  + Copy + Clone, const B: usize, const M: usize>  BPTreeCollIterator<T, B, M> {
    pub fn new(coll: BpTreeCollection<T, B, M>) -> Self {
        let curr_index = 0usize;

        Self { coll, curr_index }
    }

    pub fn increase_index(&mut self) {
        match self.curr_index > 1023 {
            true => self.curr_index = 0,
            false => self.curr_index += 1,
        }
    }


    pub fn get_at_index(&self) -> T {
        self.coll.get_at(self.curr_index).clone()
    }
}


impl<T: Init  + Copy + Clone, const B: usize, const M: usize> std::iter::Iterator for BPTreeCollIterator<T, B, M> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.get_at_index();
        self.increase_index();

        Some(item)
    }
}




impl<T: Init  + Copy + Clone, const B: usize, const M: usize> std::iter::IntoIterator for BpTreeCollection<T, B, M> {
    type Item = T;

    type IntoIter = BPTreeCollIterator<T, B, M>;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}

