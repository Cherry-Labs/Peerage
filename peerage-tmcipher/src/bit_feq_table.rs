#[derive(Clone, Copy)]
pub struct NibbleFreqTable {
    a: usize,
    b: usize,
    c: usize,
    d: usize,
}

impl NibbleFreqTable {
    pub fn unravel(&self) -> Vec<usize> {
        vec![self.a, self.b, self.c, self.d]
    }

    pub fn from_usize_vec(v: Vec<usize>) -> Self {
        assert_eq!(v.len(), 4);

        Self { a: v[0], b: v[1], c: v[2], d: v[3] }
    }

    pub fn from_enc_str(s: String) -> Self {
        
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
pub struct ByteFreqTable {
    a: NibbleFreqTable,
    b: NibbleFreqTable,
}

impl ByteFreqTable {
    pub fn unravel(&self) -> Vec<usize> {
        vec![self.a.unravel(), self.b.unravel()]
            .into_iter()
            .flatten()
            .collect::<Vec<usize>>()
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
pub struct QuadrupleWordFreqTable {
    a: ByteWordFreqTable,
    b: ByteWordFreqTable,
    c: ByteWordFreqTable,
    d: ByteWordFreqTable,
}


impl QuadrupleWordFreqTable {
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