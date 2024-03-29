use std::marker::PhantomData;

#[derive(Clone, Copy)]
pub struct NibbleFreqTable {
    a: usize,
    b: usize,
    c: usize,
    d: usize,
}

impl NibbleFreqTable {
    pub fn new_random() -> Self {
        Self::from_usize_vec(vec![crate::rng::rand_usize(); 4])
    }

    pub fn unravel(&self) -> Vec<usize> {
        vec![self.a, self.b, self.c, self.d]
    }
    
    pub fn new_zeros() -> Self {
        Self::from_usize_vec(vec![0; 4])
    }

    pub fn from_usize_vec(v: Vec<usize>) -> Self {
        assert_eq!(v.len(), 4);

        Self { a: v[0], b: v[1], c: v[2], d: v[3] }
    }

    pub fn from_enc_str(s: String) -> Self {
        let v = s.split(".")
                            .map(|x| {
                                x.chars().map(|x| x.is_numeric());

                                usize::from_str_radix(&format!("{x}"), 10).unwrap()
                            })
                            .collect::<Vec<usize>>();


        Self::from_usize_vec(v)
    }
}

impl std::ops::Index<usize> for NibbleFreqTable {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.a,
            1 => &self.b,
            2 => &self.c,
            3 => &self.d,
            _ => panic!("Index should not be larger than 3")

        }
    }
}

impl std::ops::IndexMut<usize> for NibbleFreqTable {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.a,
            1 => &mut self.b,
            2 => &mut self.c,
            3 => &mut self.d,
            _ => panic!("Index should not be larger than 3")

        }
    }
}

#[derive(Clone, Copy)]
pub struct SessetFreqTable {
    a: usize,
    b: usize,
    c: usize,
    d: usize,
    e: usize,
    f: usize,
}

impl SessetFreqTable {
    pub fn new_random() -> Self {
        Self::from_usize_vec(vec![crate::rng::rand_usize(); 6])
    }

    pub fn unravel(&self) -> Vec<usize> {
        vec![
            self.a, 
            self.b, 
            self.c, 
            self.d,
            self.e,
            self.f,
        ]
    }
    
    pub fn new_zeros() -> Self {
        Self::from_usize_vec(vec![0; 6])
    }

    pub fn from_usize_vec(v: Vec<usize>) -> Self {
        assert_eq!(v.len(), 6);

        Self { 
            a: v[0], 
            b: v[1], 
            c: v[2], 
            d: v[3],
            e: v[4],
            f: v[5]
        }
    }

    pub fn from_enc_str(s: String) -> Self {
        let v = s.split(".")
                            .map(|x| {
                                x.chars().map(|x| x.is_numeric());

                                usize::from_str_radix(&format!("{x}"), 10).unwrap()
                            })
                            .collect::<Vec<usize>>();


        Self::from_usize_vec(v)
    }
}

impl std::ops::Index<usize> for SessetFreqTable {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.a,
            1 => &self.b,
            2 => &self.c,
            3 => &self.d,
            4 => &self.e,
            5 => &self.f,
            _ => panic!("Index should not be larger than 5")

        }
    }
}

impl std::ops::IndexMut<usize> for SessetFreqTable {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.a,
            1 => &mut self.b,
            2 => &mut self.c,
            3 => &mut self.d,
            4 => &mut self.e,
            5 => &mut self.f,
            _ => panic!("Index should not be larger than 5")

        }
    }
}

#[derive(Clone, Copy)]
pub struct ByteFreqTable {
    a: NibbleFreqTable,
    b: NibbleFreqTable,
}

impl ByteFreqTable {
    pub fn new_random() -> Self {
        Self::from_usize_vec(vec![crate::rng::rand_usize(); 8])
    }

    pub fn unravel(&self) -> Vec<usize> {
        vec![self.a.unravel(), self.b.unravel()]
            .into_iter()
            .flatten()
            .collect::<Vec<usize>>()
    }

    pub fn new_zeros() -> Self {
        Self::from_usize_vec(vec![0; 8])
    }

    pub fn from_nibbles(v: Vec<NibbleFreqTable>) -> Self {
        assert_eq!(v.len(), 2);

        Self { a: v[0], b: v[1] }
    }

    pub fn from_usize_vec(v: Vec<usize>) -> Self {
        assert_eq!(v.len(), 8);

        let mut v_nibbles: Vec<NibbleFreqTable> = vec![];

        for i in (0..8).step_by(4) {
            let curr_v = v[i..i + 4].to_vec();

            let nibble_freq = NibbleFreqTable::from_usize_vec(curr_v);

            v_nibbles.push(nibble_freq);
        }

        Self::from_nibbles(v_nibbles)
    }

    pub fn from_enc_str(s: String) -> Self {
        let v = s.split(".")
                            .map(|x| {
                                x.chars().map(|x| x.is_numeric());

                                usize::from_str_radix(&format!("{x}"), 10).unwrap()
                            })
                            .collect::<Vec<usize>>();


        Self::from_usize_vec(v)
    }

}

impl std::ops::Index<usize> for ByteFreqTable {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        match index > 4 {
            true => &self.b[index % 4],
            false => &self.a[index]
        }
    }    
}

impl std::ops::IndexMut<usize> for ByteFreqTable {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index > 7 {
            panic!("Index should not be larger than 7.")
        }

        match index > 3 {
            true => &mut self.b[index % 3],
            false => &mut self.a[index]
        }
    }
}




#[derive(Clone, Copy)]
pub struct ByteWordFreqTable {
    a: ByteFreqTable,
    b: ByteFreqTable,
    c: ByteFreqTable,
    d: ByteFreqTable,
}

impl ByteWordFreqTable {
    pub fn new_random() -> Self {
        Self::from_usize_vec(vec![crate::rng::rand_usize(); 32])
    }

    pub fn unravel(&self) -> Vec<usize> {
        vec![self.a,
                self.b,
                self.c,
                self.d]
                    .into_iter()
                    .map(|x| x.unravel())
                    .flatten()
                    .collect::<Vec<usize>>()
    }

    pub fn new_zeros() -> Self {
        Self::from_usize_vec(vec![0; 32])
    }

    pub fn from_bytes(v: Vec<ByteFreqTable>) -> Self {
        assert_eq!(v.len(), 4);

        Self { a: v[0], b: v[1], c: v[2], d: v[3] }
    }

    pub fn from_usize_vec(v: Vec<usize>) -> Self {
        assert_eq!(v.len(), 32);

        let mut v_bytes: Vec<ByteFreqTable> = vec![];

        for i in (0..32).step_by(8) {
            let curr_v = v[i..i + 8].to_vec();

            let byte_freq = ByteFreqTable::from_usize_vec(curr_v);

            v_bytes.push(byte_freq);
        }

        Self::from_bytes(v_bytes)
    }

    pub fn from_enc_str(s: String) -> Self {
        let v = s.split(".")
                            .map(|x| {
                                x.chars().map(|x| x.is_numeric());

                                usize::from_str_radix(&format!("{x}"), 10).unwrap()
                            })
                            .collect::<Vec<usize>>();


        Self::from_usize_vec(v)
    }
}

impl std::ops::Index<usize> for ByteWordFreqTable {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        if index > 0 && index < 8 {
            &self.a[index]
        } else if index > 8 && index < 16 {
            &self.b[index % 8]
        } else if index > 16 && index < 24 {
            &self.c[index % 8]
        } else if index > 24 && index < 32 {
            &self.d[index % 8]
        } else {
            panic!("Index must not be larger than 31")
        }
    }
}

impl std::ops::IndexMut<usize> for ByteWordFreqTable {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index > 0 && index < 8 {
            &mut self.a[index]
        } else if index > 8 && index < 16 {
            &mut self.b[index % 8]
        } else if index > 16 && index < 24 {
            &mut self.c[index % 8]
        } else if index > 24 && index < 32 {
            &mut self.d[index % 8]
        } else {
            panic!("Index must not be larger than 31")
        }
    }
}

#[derive(Clone, Copy)]
pub struct DoubleWordFreqTable {
    a: ByteWordFreqTable,
    b: ByteWordFreqTable,
}


impl DoubleWordFreqTable {
    pub fn new_random() -> Self {
        Self::from_usize_vec(vec![crate::rng::rand_usize(); 64])
    }

    
    pub fn unravel(&self) -> Vec<usize> {
        vec![
                self.a,
                self.b,
            ]
            .into_iter()
            .map(|x| x.unravel())
            .flatten()
            .collect::<Vec<usize>>()
    }

    pub fn new_zeros() -> Self {
        Self::from_usize_vec(vec![0; 64])
    }

    pub fn from_byte_words(v: Vec<ByteWordFreqTable>) -> Self {
        assert_eq!(v.len(), 4);

        Self { a: v[0], b: v[1] }
    }

    pub fn from_usize_vec(v: Vec<usize>) -> Self {
        assert_eq!(v.len(), 64);

        let mut v_byte_words: Vec<ByteWordFreqTable> = vec![];

        for i in (0..64).step_by(32) {
            let curr_v = v[i..i + 32].to_vec();

            let byte_word_freq = ByteWordFreqTable::from_usize_vec(curr_v);

            v_byte_words.push(byte_word_freq);
        }

        Self::from_byte_words(v_byte_words)
    }

    pub fn from_enc_str(s: String) -> Self {
        let v = s.split(".")
                            .map(|x| {
                                x.chars().map(|x| x.is_numeric());

                                usize::from_str_radix(&format!("{x}"), 10).unwrap()
                            })
                            .collect::<Vec<usize>>();


        Self::from_usize_vec(v)
    }
}

impl std::ops::Index<usize> for DoubleWordFreqTable {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        match index > 32 {
            true => &self.b[index % 32],
            false => &self.a[index]
        }
    }
}


impl std::ops::IndexMut<usize> for DoubleWordFreqTable {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index > 32 {
            true => &mut self.b[index % 32],
            false => &mut self.a[index]
        }
    }
}

#[derive(Clone, Copy)]
pub struct QuadrupleWordFreqTable {
    a: ByteWordFreqTable,
    b: ByteWordFreqTable,
    c: ByteWordFreqTable,
    d: ByteWordFreqTable,
}


impl QuadrupleWordFreqTable {
    pub fn new_random() -> Self {
        Self::from_usize_vec(vec![crate::rng::rand_usize(); 128])
    }

    
    pub fn unravel(&self) -> Vec<usize> {
        vec![
                self.a,
                self.b,
                self.c,
                self.d,
            ]
            .into_iter()
            .map(|x| x.unravel())
            .flatten()
            .collect::<Vec<usize>>()
    }

    pub fn new_zeros() -> Self {
        Self::from_usize_vec(vec![0; 128])
    }

    pub fn from_byte_words(v: Vec<ByteWordFreqTable>) -> Self {
        assert_eq!(v.len(), 4);

        Self { a: v[0], b: v[1], c: v[2], d: v[3] }
    }

    pub fn from_usize_vec(v: Vec<usize>) -> Self {
        assert_eq!(v.len(), 128);

        let mut v_byte_words: Vec<ByteWordFreqTable> = vec![];

        for i in (0..128).step_by(32) {
            let curr_v = v[i..i + 32].to_vec();

            let byte_word_freq = ByteWordFreqTable::from_usize_vec(curr_v);

            v_byte_words.push(byte_word_freq);
        }

        Self::from_byte_words(v_byte_words)
    }

    pub fn from_enc_str(s: String) -> Self {
        let v = s.split(".")
                            .map(|x| {
                                x.chars().map(|x| x.is_numeric());

                                usize::from_str_radix(&format!("{x}"), 10).unwrap()
                            })
                            .collect::<Vec<usize>>();


        Self::from_usize_vec(v)
    }
}

impl std::ops::Index<usize> for QuadrupleWordFreqTable {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        if index > 0 && index < 32 {
            &self.a[index]
        } else if index > 32 && index < 64 {
            &self.b[index % 32]
        } else if index > 64 && index < 96 {
            &self.c[index % 32]
        } else if index > 96 && index < 128 {
            &self.d[index % 32]
        } else {
            panic!("Index must not be larger than 127")
        }
    }
}


impl std::ops::IndexMut<usize> for QuadrupleWordFreqTable {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index > 0 && index < 32 {
            &mut self.a[index]
        } else if index > 32 && index < 64 {
            &mut self.b[index % 32]
        } else if index > 64 && index < 96 {
            &mut self.c[index % 32]
        } else if index > 96 && index < 128 {
            &mut self.d[index % 32]
        } else {
            panic!("Index must not be larger than 127")
        }
    }
}

pub trait Indexer: Copy + Clone +std::ops::Index<
        usize,
        Output = usize,
        > + 
        std::ops::IndexMut<
        usize> {}

#[derive(Clone, Copy)]
pub struct FreqIter<T: Indexer, const M: usize> {
    object: T,
    index: usize,
    index_impl: usize, 
}

impl<T: Indexer, const M: usize> FreqIter<T, M> {
    pub fn into_iter(object: T) -> Self {
        Self { object, index: 0usize, index_impl: 0usize }
    }
    
    pub fn get_index(&self) -> usize {
        self.object[self.index]
    }

    pub fn increase_index(&mut self) {
        match self.index == M - 1 {
            true => self.index = 0,
            false => self.index += 1,
        }
    }

    pub fn index_increase_impl(&mut self) {
        self.index_impl += 1;
    }

    pub fn get_middle(&self) -> usize {
        self.object[M / 2]
    }

    pub fn get_first(&self) -> usize {
        self.object[0]
    }

    pub fn get_last(&self) -> usize {
        self.object[M - 1]
    }

    pub fn get_quarters(&self) -> usize {
        self.object[M / 4]
    }

    pub fn get_three_fourths(&self) -> usize {
        self.object[3 * (M / 4)]
    }
}

impl<T: Indexer, const M: usize> std::iter::Iterator for FreqIter<T, M> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.get_index();
        self.increase_index();
        self.index_increase_impl();
        
        match self.index % M == 0 {
            true => None,
            false => Some(item),
        }
    }
}

impl Indexer for NibbleFreqTable {}
impl Indexer for SessetFreqTable {}
impl Indexer for ByteFreqTable {}
impl Indexer for ByteWordFreqTable {}
impl Indexer for DoubleWordFreqTable {}
impl Indexer for QuadrupleWordFreqTable {}

impl<'a> std::iter::IntoIterator for NibbleFreqTable {
    type Item = usize;

    type IntoIter = FreqIter<NibbleFreqTable, 4usize>;

    fn into_iter(self) -> Self::IntoIter {
        FreqIter::into_iter(self)
    }
}

impl std::iter::IntoIterator for SessetFreqTable {
    type Item = usize;

    type IntoIter = FreqIter<SessetFreqTable, 6usize>;

    fn into_iter(self) -> Self::IntoIter {
        FreqIter::into_iter(self)
    }
}

impl std::iter::IntoIterator for ByteFreqTable {
    type Item = usize;

    type IntoIter = FreqIter<ByteFreqTable, 8usize>;

    fn into_iter(self) -> Self::IntoIter {
        FreqIter::into_iter(self)
    }
}


impl std::iter::IntoIterator for ByteWordFreqTable {
    type Item = usize;

    type IntoIter = FreqIter<ByteWordFreqTable, 32usize>;

    fn into_iter(self) -> Self::IntoIter {
        FreqIter::into_iter(self)
    }
}

impl std::iter::IntoIterator for DoubleWordFreqTable {
    type Item = usize;

    type IntoIter = FreqIter<DoubleWordFreqTable, 64usize>;

    fn into_iter(self) -> Self::IntoIter {
        FreqIter::into_iter(self)
    }
}

impl std::iter::IntoIterator for QuadrupleWordFreqTable {
    type Item = usize;

    type IntoIter = FreqIter<QuadrupleWordFreqTable, 128usize>;

    fn into_iter(self) -> Self::IntoIter {
        FreqIter::into_iter(self)
    }
}

