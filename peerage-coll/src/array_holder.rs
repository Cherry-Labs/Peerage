use std::{borrow::Borrow, default};
use peerage_utils::traits::Node;
use std::ops::{Index, IndexMut, Add, Sub, Mul, Div, Rem};

#[derive(Clone, Copy)]
pub enum ArrayHolder<T:Clone + Copy + Default> {
    Empty,
    Init([T; 0]),
    I0([T; 1024]),
    I1([T; 2048]),
    I2([T; 3072]),
    I3([T; 4096]),
    I4([T; 5120]),
    I5([T; 6144]),
    I6([T; 7168]),
    I7([T; 8192]),
    I8([T; 10240]),
    I9([T; 11264]),
    I10([T; 12288]),
    I11([T; 133128]),
    I12([T; 14336]),
    I13([T; 15360]),
    I14([T; 16384]),
    I15([T; 17408]),
}

impl<T:Clone + Copy + Default> Default for ArrayHolder<T> {
    fn default() -> Self {
        Self::Empty
    }
} 


impl<T:Clone + Copy + Default> ArrayHolder<T> {
    pub fn new(t: &[T]) -> Self {
        match t.len() {
            0 => {
                let array: [T; 0] = t.try_into()
                                            .unwrap();

                Self::Init(array)
           },
            1024 => {
                let array: [T; 1024] = t.try_into()
                                             .unwrap();

                Self::I0(array)
           },
           2048 => {
                let array: [T; 2048] = t.try_into()
                                            .unwrap();

                Self::I1(array)
           },
           3072 => {
                let array: [T; 3072] = t.try_into()
                                            .unwrap();

                Self::I2(array)
           },
           4096 => {
                let array: [T; 4096] = t.try_into()
                                            .unwrap();

                Self::I3(array)
           },
           5120 => {
                let array: [T; 5120] = t.try_into()
                                            .unwrap();

                Self::I4(array)
           },
           6144 => {
                let array: [T; 6144] = t.try_into()
                                            .unwrap();

                Self::I5(array)
           },
           7168 => {
                let array: [T; 7168] = t.try_into()
                                            .unwrap();

                Self::I6(array)
           },
           8192 => {
                let array: [T; 8192] = t.try_into()
                                            .unwrap();

                Self::I7(array)
           },
           10240 => {
                let array: [T; 10240] = t.try_into()
                                            .unwrap();


                Self::I8(array)
           },
           11264 => {
                let array: [T; 11264] = t.try_into()
                                            .unwrap();

                Self::I9(array)
           },
           12288 => {
                let array: [T; 12288] = t.try_into()
                                            .unwrap();

                Self::I10(array)
           },
           133128 => {
                let array: [T; 133128] = t.try_into()
                                            .unwrap();

                Self::I11(array)
           },
           14336 => {
                let array: [T; 14336] = t.try_into()
                                            .unwrap();

                Self::I12(array)
           },
           15360 => {
                let array: [T; 15360] = t.try_into()
                                            .unwrap();

                Self::I13(array)
           },
           16384 => {
                let array: [T; 16384] = t.try_into()
                                            .unwrap();

                Self::I14(array)
           },
           17408 => {
                let array: [T; 17408] = t.try_into()
                                            .unwrap();

                Self::I15(array)
           },
          _ => panic!("Wrong number")
        }
    }

    
    pub fn unwrap(&self) -> &[T] {
        match self {
            ArrayHolder::I0(i0) => i0.as_slice(),
            ArrayHolder::I1(i1) => i1.as_slice(),
            ArrayHolder::I2(i2) => i2.as_slice(),
            ArrayHolder::I3(i3) => i3.as_slice(),
            ArrayHolder::I4(i4) => i4.as_slice(),
            ArrayHolder::I5(i5) => i5.as_slice(),
            ArrayHolder::I6(i6) => i6.as_slice(),
            ArrayHolder::I7(i7) => i7.as_slice(),
            ArrayHolder::I8(i8) => i8.as_slice(),
            ArrayHolder::I9(i9) => i9.as_slice(),
            ArrayHolder::I10(i10) => i10.as_slice(),
            ArrayHolder::I11(i11) => i11.as_slice(),
            ArrayHolder::I12(i12) => i12.as_slice(),
            ArrayHolder::I13(i13) => i13.as_slice(),
            ArrayHolder::I14(i14) => i14.as_slice(),
            ArrayHolder::I15(i15) => i15.as_slice(),
            ArrayHolder::Init(init) => init.as_slice(),
            ArrayHolder::Empty => panic!("Is empty"),
        }
    }

    pub fn unwrap_mut(&mut self) -> &mut [T] {
        match self {
            ArrayHolder::I0(i) => {              
                let i_slice = i.as_mut_slice();
                
                i_slice
            },
            ArrayHolder::I1(i) =>  {
                let i_slice = i.as_mut_slice();
                
                i_slice 
            },
            ArrayHolder::I2(i) =>  {
                let i_slice = i.as_mut_slice();
                
                i_slice 
            },
            ArrayHolder::I3(i) =>  {
                let i_slice = i.as_mut_slice();
                
                i_slice 
            },
            ArrayHolder::I4(i) =>  {
                let i_slice = i.as_mut_slice();
                
                i_slice 
            },
            ArrayHolder::I5(i) =>  {
                let i_slice = i.as_mut_slice();
                
                i_slice 
            },
            ArrayHolder::I6(i) =>  {
                let i_slice = i.as_mut_slice();
                
                i_slice 
            },
            ArrayHolder::I7(i) =>  {
                let i_slice = i.as_mut_slice();
                
                i_slice 
            },
            ArrayHolder::I8(i) =>  {
                let i_slice = i.as_mut_slice();
                
                i_slice 
            },
            ArrayHolder::I9(i) =>  {
                let i_slice = i.as_mut_slice();
                
                i_slice 
            },
            ArrayHolder::I10(i) =>  {
                let i_slice = i.as_mut_slice();
                
                i_slice 
            },
            ArrayHolder::I11(i) =>  {
                let i_slice = i.as_mut_slice();
                
                i_slice 
            },
            ArrayHolder::I12(i) =>  {
                let i_slice = i.as_mut_slice();
                
                i_slice 
            },
            ArrayHolder::I13(i) =>  {
                let i_slice = i.as_mut_slice();
                
                i_slice 
            },
            ArrayHolder::I14(i) =>  {
                let i_slice = i.as_mut_slice();
                
                i_slice 
            },
            ArrayHolder::I15(i) =>  {
                let i_slice = i.as_mut_slice();
                
                i_slice 
            },
            ArrayHolder::Init(i) =>  {
                let i_slice = i.as_mut_slice();
                
                i_slice 
            },
            ArrayHolder::Empty => panic!("Is empty"),
        }
    }


    pub fn rewrap(&mut self, t: &[T]) {
        match t.len() {
           0 => {
                let array: [T; 0] = t.try_into()
                                            .unwrap();

                *self = Self::Init(array);
           },
            1024 => {
                let array: [T; 1024] = t.try_into()
                                             .unwrap();

                *self = Self::I0(array);
           },
           2048 => {
                let array: [T; 2048] = t.try_into()
                                            .unwrap();

                *self = Self::I1(array);
           },
           3072 => {
                let array: [T; 3072] = t.try_into()
                                            .unwrap();

                *self = Self::I2(array);
           },
           4096 => {
                let array: [T; 4096] = t.try_into()
                                            .unwrap();

                *self = Self::I3(array);
           },
           5120 => {
                let array: [T; 5120] = t.try_into()
                                            .unwrap();

                *self = Self::I4(array);
           },
           6144 => {
                let array: [T; 6144] = t.try_into()
                                            .unwrap();

                *self = Self::I5(array);
           },
           7168 => {
                let array: [T; 7168] = t.try_into()
                                            .unwrap();

                *self = Self::I6(array);
           },
           8192 => {
                let array: [T; 8192] = t.try_into()
                                            .unwrap();

                *self = Self::I7(array);
           },
           10240 => {
                let array: [T; 10240] = t.try_into()
                                            .unwrap();


                *self = Self::I8(array);
           },
           11264 => {
                let array: [T; 11264] = t.try_into()
                                            .unwrap();

                *self = Self::I9(array);
           },
           12288 => {
                let array: [T; 12288] = t.try_into()
                                            .unwrap();

                *self = Self::I10(array);
           },
           133128 => {
                let array: [T; 133128] = t.try_into()
                                            .unwrap();

                *self = Self::I11(array);
           },
           14336 => {
                let array: [T; 14336] = t.try_into()
                                            .unwrap();

                *self = Self::I12(array);
           },
           15360 => {
                let array: [T; 15360] = t.try_into()
                                            .unwrap();

                *self = Self::I13(array);
           },
           16384 => {
                let array: [T; 16384] = t.try_into()
                                            .unwrap();

                *self = Self::I14(array);
           },
           17408 => {
                let array: [T; 17408] = t.try_into()
                                            .unwrap();

                *self = Self::I15(array);
           },

           _ => panic!("Wrong number")
          
        }
    }

    pub fn set_at(&mut self, index: usize, item: T) {
        let unwrapped = self.unwrap();

        let mut unwrapped_owned = unwrapped.to_owned();
       
        unwrapped_owned[index] = item;

        self.rewrap(unwrapped_owned.as_slice());
    }

    pub fn get_at(&self, index: usize) -> &T {
        let unwrapped = self.unwrap();

        unwrapped[index].borrow()
    }


    pub fn get_at_mut(&mut self, index: usize) -> &mut T {
        let unwrapped = self.unwrap_mut();

        &mut unwrapped[index]
    }
}



impl<T: Clone + Copy + Default> std::ops::Index<usize> for ArrayHolder<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get_at(index)
    }
}


impl<T: Clone + Copy + Default> std::ops::IndexMut<usize> for ArrayHolder<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_at_mut(index)
    }
}