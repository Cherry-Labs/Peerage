use peerage_codecs::nibble_codec::NibbleCodec;
use peerage_utils::bin_utils::{Byte, ByteWord, DoubleWord, Nibble, QuadrupleWord, Sesset};

#[derive(Clone, Copy)]
pub struct KeyPhrase {
    quadruple_words: [QuadrupleWord; 128],
    double_words: [DoubleWord; 64],
    byte_words: [ByteWord; 32],
    bytes: [Byte; 8],
    sessets: [Sesset; 6],
    nibbles: [Nibble; 4],
}

impl KeyPhrase {
    pub fn new() -> Self {
        let rand_codec = NibbleCodec::new_random();

        let qw_rand = rand_codec.get_qw().into_vec();
        let dw_rand = rand_codec.get_dw().into_vec();
        let bw_rand = rand_codec.get_bw().into_vec();
        let by_rand = rand_codec.get_by().into_vec();
        let se_rand = rand_codec.get_se().into_vec();
        let ni_rand = rand_codec.get_ni().into_vec();

        let mut quadruple_words = [QuadrupleWord::default(); 128];
        let mut double_words = [DoubleWord::default(); 64];
        let mut byte_words = [ByteWord::default(); 32];
        let mut bytes = [Byte::default(); 8];
        let mut sessets = [Sesset::default(); 6];
        let mut nibbles = [Nibble::default(); 4];

        for i in 0usize..128usize {
            quadruple_words[i] = qw_rand[i];
        }

        for i in 0usize..64usize {
            double_words[i] = dw_rand[i];
        }

        for i in 0usize..32usize {
            byte_words[i] = bw_rand[i];
        }

        for i in 0usize..8usize {
            bytes[i] = by_rand[i];
        }

        for i in 0usize..6usize {
            sessets[i] = se_rand[i];
        }

        for i in 0usize..4usize {
            nibbles[i] = ni_rand[i];
        }

        Self {
            quadruple_words,
            double_words,
            byte_words,
            bytes,
            sessets,
            nibbles,
        }
    }

    pub fn from_encoded(s: String) -> Self {
        let rand_codec = NibbleCodec::decodec(s);

        let qw_rand = rand_codec.get_qw().into_vec();
        let dw_rand = rand_codec.get_dw().into_vec();
        let bw_rand = rand_codec.get_bw().into_vec();
        let by_rand = rand_codec.get_by().into_vec();
        let se_rand = rand_codec.get_se().into_vec();
        let ni_rand = rand_codec.get_ni().into_vec();

        let mut quadruple_words = [QuadrupleWord::default(); 32];
        let mut byte_words = [ByteWord::default(); 16];
        let mut bytes = [Byte::default(); 8];
        let mut nibbles = [Nibble::default(); 4];

        let mut quadruple_words = [QuadrupleWord::default(); 128];
        let mut double_words = [DoubleWord::default(); 64];
        let mut byte_words = [ByteWord::default(); 32];
        let mut bytes = [Byte::default(); 8];
        let mut sessets = [Sesset::default(); 6];
        let mut nibbles = [Nibble::default(); 4];

        for i in 0usize..128usize {
            quadruple_words[i] = qw_rand[i];
        }

        for i in 0usize..64usize {
            double_words[i] = dw_rand[i];
        }

        for i in 0usize..32usize {
            byte_words[i] = bw_rand[i];
        }

        for i in 0usize..8usize {
            bytes[i] = by_rand[i];
        }

        for i in 0usize..6usize {
            sessets[i] = se_rand[i];
        }

        for i in 0usize..4usize {
            nibbles[i] = ni_rand[i];
        }

        Self {
            quadruple_words,
            double_words,
            byte_words,
            bytes,
            sessets,
            nibbles,
        }
    }

    pub fn encode(&self) -> String {
        let quadruple_words = self.quadruple_words.to_vec();
        let double_words = self.double_words.to_vec();
        let byte_words = self.byte_words.to_vec();
        let bytes = self.bytes.to_vec();
        let sessets = self.sessets.to_vec();
        let nibbles = self.nibbles.to_vec();

        let codec = NibbleCodec::from_vec(
            quadruple_words,
            double_words,
            byte_words,
            bytes,
            sessets,
            nibbles,
        );

        codec.encode()
    }
}
