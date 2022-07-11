
use peerage_utils::pub_traits::Tree;
use peerage_utils::bin_utils::QuadrupleWord;
use crate::degee::Degree;


#[derive(Clone, Copy)]
pub struct BpTreeCollection<T: Tree  + Copy + Clone, const B: usize, const M: usize> {
   array: [T; B],
   current_buffer: [T; M],
}

impl<T: Tree  + Copy + Clone, const B: usize, const M: usize> BpTreeCollection<T, B, M>  {
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

    pub fn binary_search(&self, value: T) -> isize {
       let mut low = 0usize;
       let mut high = B.clone();

       loop {
            let mid = low + (high - low) / 2;

            if low <= high {
                return mid as isize;
            }

            if self.array[mid].is_equal_to(value){
                break mid;
            } else if self.array[mid].is_lesser_than(value) {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
       };

       -1
    }
}


impl<T: Tree  + Copy + Clone, const B: usize, const M: usize>  std::ops::Index<usize> for BpTreeCollection<T, B, M> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index > 1023 {
            true => panic!("Size of BPTreeCollection is 1024. Index ends at 1023."),
            false => self.get_at(index),
        }
    }
}


pub struct BPTreeCollIterator<T: Tree  + Copy + Clone, const B: usize, const M: usize> {
    coll: BpTreeCollection<T, B, M>,
    curr_index: usize
}


impl<T: Tree  + Copy + Clone, const B: usize, const M: usize>  BPTreeCollIterator<T, B, M> {
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


impl<T: Tree  + Copy + Clone, const B: usize, const M: usize> std::iter::Iterator for BPTreeCollIterator<T, B, M> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.get_at_index();
        self.increase_index();

        Some(item)
    }
}




impl<T: Tree  + Copy + Clone, const B: usize, const M: usize> std::iter::IntoIterator for BpTreeCollection<T, B, M> {
    type Item = T;

    type IntoIter = BPTreeCollIterator<T, B, M>;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}


#[derive(Clone, Copy)]
pub enum CollWrapper<T: Tree  + Copy + Clone> {
    Two(BpTreeCollection<T, 2, 1>),
    Four(BpTreeCollection<T, 4, 2>),
    Sixteen(BpTreeCollection<T, 16, 8>),
    TwoFiftySix(BpTreeCollection<T, 256, 64>),
    FiveTwelve(BpTreeCollection<T, 512, 128>),
    TenTwentyFour(BpTreeCollection<T, 1024, 256>),
    TwoFourtyEight(BpTreeCollection<T, 2048, 512>),
    FortyNightySix(BpTreeCollection<T, 4096, 1024>),
}


impl<T: Tree  + Copy + Clone> CollWrapper<T> {
    pub fn new(degree: Degree) -> Self {
        let item = match degree {
            Degree::Two => {
                const M: usize = 2;
                const B: usize = 1;
                Self::Two(BpTreeCollection::<T, M, B>::new())
            },
            Degree::Four => {
                const M: usize = 4;
                const B: usize = 2;
                Self::Four(BpTreeCollection::<T, M, B>::new())
            },
            Degree::Sixteen => {
                const M: usize = 16;
                const B: usize = 8;
                Self::Sixteen(BpTreeCollection::<T, M, B>::new())
            },
            Degree::TwoFiftySix => {
                const M: usize = 256;
                const B: usize = 64;
                Self::TwoFiftySix(BpTreeCollection::<T, M, B>::new())
            },
            Degree::FiveTwelve => {
                const M: usize = 512;
                const B: usize = 128;
                Self::FiveTwelve(BpTreeCollection::<T, M, B>::new())
            },
            Degree::TenTwnentyFour => {
                const M: usize = 1024;
                const B: usize = 256;
                Self::TenTwentyFour(BpTreeCollection::<T, M, B>::new())
            },
            Degree::TwoFourtyEight => {
                const M: usize = 2048;
                const B: usize = 512;
                Self::TwoFourtyEight(BpTreeCollection::<T, M, B>::new())
            },
            Degree::FortyNightySix => {
                const M: usize = 4096;
                const B: usize = 1024;
                Self::FortyNightySix(BpTreeCollection::<T, M, B>::new())
            },
        };

        item
    }

    pub fn return_self_two(&self) -> BpTreeCollection<T, 2, 1> {
        if let Self::Two(item) = self {
            let mut  item_deref =  item.clone();
            item_deref

        } else {
            panic!("Problem with getting coll")
        }


    }

    pub fn return_ref_two(&self) -> &BpTreeCollection<T, 2, 1> {
        if let Self::Two(item) = self {
            item

        } else {
            panic!("Problem with getting coll")
        }
    }

    pub fn return_mut_two(&mut self) -> &mut BpTreeCollection<T, 2, 1> {
        if let Self::Two(item) = self {
            
            item

        } else {
            panic!("Problem with getting coll")
        }
    }


    pub fn return_self_four(&self) -> BpTreeCollection<T, 4, 2> {
        if let Self::Four(item) = self {
            let mut  item_deref =  item.clone();
            item_deref

        } else {
            panic!("Problem with getting coll")
        }
    }

    pub fn return_ref_four(&self) -> &BpTreeCollection<T, 4, 2> {
        if let Self::Four(item) = self {
            item

        } else {
            panic!("Problem with getting coll")
        }
    }

    pub fn return_mut_four(&mut self) -> &mut BpTreeCollection<T, 4, 2> {
        if let Self::Four(item) = self {

            item

        } else {
            panic!("Problem with getting coll")
        }
    }


    pub fn return_self_sixteen(&self) -> BpTreeCollection<T, 16, 8> {
        if let Self::Sixteen(item) = self {
            let mut  item_deref =  item.clone();
            item_deref

        } else {
            panic!("Problem with getting coll")
        }
    }

    pub fn return_ref_sixteen(&self) -> &BpTreeCollection<T, 16, 8> {
        if let Self::Sixteen(item) = self {
            item

        } else {
            panic!("Problem with getting coll")
        }
    }

    pub fn return_mut_sixteen(&mut self) -> &mut BpTreeCollection<T, 16, 8> {
        if let Self::Sixteen(item) = self {
            item

        } else {
            panic!("Problem with getting coll")
        }
    }

    
    pub fn return_self_two_fifty_six(&self) -> BpTreeCollection<T, 256, 64> {
        if let Self::TwoFiftySix(item) = self {
            let mut  item_deref =  item.clone();
            item_deref

        } else {
            panic!("Problem with getting coll")
        }
    }

    pub fn return_ref_two_fifty_six(&self) -> &BpTreeCollection<T, 256, 64> {
        if let Self::TwoFiftySix(item) = self {
            item

        } else {
            panic!("Problem with getting coll")
        }
    }

    pub fn return_mut_two_fifty_six(&mut self) -> &mut BpTreeCollection<T, 256, 64> {
        if let Self::TwoFiftySix(item) = self {
            item

        } else {
            panic!("Problem with getting coll")
        }
    }


    pub fn return_self_five_twelve(&self) -> BpTreeCollection<T, 512, 128> {
        if let Self::FiveTwelve(item) = self {
            let mut  item_deref =  item.clone();
            item_deref

        } else {
            panic!("Problem with getting coll")
        }
    }

    pub fn return_ref_five_twelve(&self) -> &BpTreeCollection<T, 512, 128> {
        if let Self::FiveTwelve(item) = self {
            item

        } else {
            panic!("Problem with getting coll")
        }
    }

    pub fn return_mut_five_twelve(&mut self) -> &mut BpTreeCollection<T, 512, 128> {
        if let Self::FiveTwelve(item) = self {

            item

        } else {
            panic!("Problem with getting coll")
        }
    }

    pub fn return_self_ten_twenty_four(&self) -> BpTreeCollection<T, 1024, 256> {
        if let Self::TenTwentyFour(item) = self {
            let mut  item_deref =  item.clone();
            item_deref

        } else {
            panic!("Problem with getting coll")
        }
    }

    pub fn return_ref_ten_twenty_four(&self) -> &BpTreeCollection<T, 1024, 256> {
        if let Self::TenTwentyFour(item) = self {
            item

        } else {
            panic!("Problem with getting coll")
        }
    }

    pub fn return_mut_ten_twenty_four(&mut self) -> &mut BpTreeCollection<T, 1024, 256> {
        if let Self::TenTwentyFour(item) = self {
            item

        } else {
            panic!("Problem with getting coll")
        }
    }

    pub fn return_self_two_fourty_eight(&self) -> BpTreeCollection<T, 2048, 512> {
        if let Self::TwoFourtyEight(item) = self {
            let mut  item_deref =  item.clone();
            item_deref

        } else {
            panic!("Problem with getting coll")
        }
    }

    pub fn return_ref_two_fourty_eight(&self) -> &BpTreeCollection<T, 2048, 512> {
        if let Self::TwoFourtyEight(item) = self {
            item

        } else {
            panic!("Problem with getting coll")
        }
    }

    pub fn return_mut_two_fourty_eight(&mut self) -> &mut BpTreeCollection<T, 2048, 512> {
        if let Self::TwoFourtyEight(item) = self {
            item

        } else {
            panic!("Problem with getting coll")
        }
    }

    pub fn return_self_forty_nighty_six(&self) -> BpTreeCollection<T, 4096, 1024> {
        if let Self::FortyNightySix(item) = self {
            let mut  item_deref =  item.clone();
            item_deref

        } else {
            panic!("Problem with getting coll")
        }
    }

    pub fn return_ref_forty_nighty_six(&self) -> &BpTreeCollection<T, 4096, 1024> {
        if let Self::FortyNightySix(item) = self {
            item

        } else {
            panic!("Problem with getting coll")
        }
    }

    pub fn return_mut_forty_nighty_six(&mut self) -> &mut BpTreeCollection<T, 4096, 1024> {
        if let Self::FortyNightySix(item) = self {
            item

        } else {
            panic!("Problem with getting coll")
        }
    }


    

}



#[derive(Clone, Copy)]
pub struct BpCollHolder<T: Tree + Clone + Copy> {
    coll_wrapper: CollWrapper<T>,
    degree: Degree,

}


impl<T: Tree + Clone + Copy> BpCollHolder<T> {
    pub fn new(degree: Degree) -> Self {
        let coll_wrapper = CollWrapper::<T>::new(degree.clone());

        Self { coll_wrapper, degree }
    }

    pub fn get_degree_num(&self) -> usize {
        self.degree.into_usize()
    }

    pub fn return_self_as_clone(&self) -> Self {
        self.clone()
    }

}



impl<T: Tree + Copy + Clone> Tree for BpCollHolder<T> { 
    type LedgerType = peerage_ledger::Ledger;
    type InputType = Degree;

    fn init() -> Self {
        Self::new(Degree::Two)
    }
    fn new(input: Degree) -> Self {
        Self::new(input)

    }
    fn mutate(&mut self, other: Self) {

    }
    fn get_ledger_id(&self) -> Self::LedgerType {
        peerage_ledger::Ledger
    }
    fn is_equal_to(&self, other: Self) -> bool {
        true
    }
    fn is_greater_than(&self, other: Self) -> bool {
        true
    }
    fn is_lesser_than(&self, other: Self) -> bool {
        true
    }

}